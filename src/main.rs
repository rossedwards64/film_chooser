use crate::record_structs::record::Record;
use crate::search::record_filter::FILTERS;
use anyhow::Result;
use search::search_records::search_records;
use std::{
    env::args,
    ffi::OsString,
    fs::{write, File},
    io::{stdin, BufReader},
    path::Path,
};

mod get_records;
mod record_structs;
mod search;

/*
 * TODO: plan a method of filtering records with the filter map
 * TODO: plan a method of searching for records using another record's reference id
 */

const SEARCH_OPTIONS: [&str; 6] = ["Title", "Director", "Episode", "Cast", "Rating", "Actor"];
const DATASETS: [&str; 7] = [
    "title.akas.tsv",
    "title.basics.tsv",
    "title.crew.tsv",
    "title.episode.tsv",
    "title.principals.tsv",
    "title.ratings.tsv",
    "name.basics.tsv",
];

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<OsString> = args().map(OsString::from).collect();
    let input = menu();
    let query = SEARCH_OPTIONS[input - 1];
    println!("Searching by {query}");
    if !&args.is_empty() && args[1] == "--file" {
        if let Ok(records) = read_local_file(&args[2]) {
            let r = search_records(&records, FILTERS.get(query).unwrap());
            r.iter().for_each(|&r| {
                println!("{}", r);
            })
        }
    } else {
        let dataset_filename = OsString::from(DATASETS[input].to_string() + ".gz");
        if let Ok(records) = download_file(dataset_filename).await {
            println!("{}", &records.first().unwrap());
        }
    }
    Ok(())
}

fn menu() -> usize {
    SEARCH_OPTIONS
        .iter()
        .enumerate()
        .map(|(i, _opt)| (i + 1, _opt))
        .for_each(|(i, opt)| {
            println!("{i}: {opt}");
        });
    print!("\nSelect a search option: ");

    let input = {
        let mut input_buf = String::new();
        match stdin().read_line(&mut input_buf) {
            Ok(_) => Ok(input_buf),
            Err(e) => Err(e),
        }
    };

    match &input {
        Ok(i) => i.trim().parse::<usize>().unwrap_or(0),
        Err(_e) => 0,
    }
}

fn read_local_file(file_path: &OsString) -> Result<Vec<Box<dyn Record>>> {
    println!("Using local file {}", file_path.to_str().expect("N/A"));
    let file = File::open(file_path)?;
    get_records::get_records_from_file(
        BufReader::new(file),
        Path::new(file_path)
            .file_name()
            .expect("Couldn't convert filename to string.")
            .to_str()
            .unwrap_or(""),
    )
}

async fn download_file(dataset_url: OsString) -> Result<Vec<Box<dyn Record>>> {
    let dataset_url = dataset_url
        .to_str()
        .expect("Couldn't convert filename to string.");
    let dataset = get_records::download_dataset(&dataset_url.to_string()).await;
    let file = dataset?.path().join(dataset_url);
    println!("Downloaded file to {}, now unzipping...", file.display());
    let bytes = get_records::decompress_content(&file)?;
    let decomp_file = Path::new(&file);
    write(decomp_file, bytes).expect("Could write to file.");
    let decomp_file = File::open(decomp_file)?;
    get_records::get_records_from_file(BufReader::new(decomp_file), dataset_url)
}
