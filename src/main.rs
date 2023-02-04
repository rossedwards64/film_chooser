use crate::record_structs::record::Record;
use crate::search::{filter_records::filter_records, record_filter::FILTERS};
use anyhow::Result;
use std::{
    env::args,
    ffi::{OsStr, OsString},
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
            if let Some(filter_func) = FILTERS.get(query) {
                let r = filter_records(&records, *filter_func);
                for r in &r {
                    println!("{r}");
                }
            };
        }
    } else {
        let dataset_filename = OsString::from(DATASETS[input].to_string() + ".gz");
        if let Ok(records) = download_file(dataset_filename).await {
            records
                .first()
                .as_ref()
                .map_or_else(|| println!("No record found!"),
                             |r| println!("{r}"));
        }
    }
    Ok(())
}

fn menu() -> usize {
    SEARCH_OPTIONS
        .iter()
        .enumerate()
        .map(|option| (option.0 + 1, option.1))
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
    let file = File::open(file_path)?;
    let file_path = {
        let path_str = Path::new(file_path)
            .file_name()
            .map_or(Some(""), OsStr::to_str);
        path_str.unwrap_or("")
    };

    Ok(get_records::parse_records_to_vec(
        BufReader::new(file),
        file_path,
    ))
}

async fn download_file(dataset_url: OsString) -> Result<Vec<Box<dyn Record>>> {
    let dataset_url = dataset_url.to_str().map_or("", |d| d);
    let dataset = get_records::download_dataset(&dataset_url.to_string()).await;
    let file = dataset?.path().join(dataset_url);
    println!("Downloaded file to {}, now unzipping...", file.display());
    let bytes = get_records::decompress_content(&file)?;
    let decomp_file = Path::new(&file);
    write(decomp_file, bytes)?;
    let decomp_file = File::open(decomp_file)?;
    Ok(get_records::parse_records_to_vec(
        BufReader::new(decomp_file),
        dataset_url,
    ))
}
