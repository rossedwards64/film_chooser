use crate::record_structs::record::Record;
use crate::search::record_filter::FILTERS;
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

const CATEGORIES: [&str; 6] = ["Title", "Director", "Episode", "Cast", "Rating", "Actor"];

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
    let filter_category = CATEGORIES[input.0 - 1];
    let query = input.1.trim_end();
    println!("Searching by {query}");
    if !&args.is_empty() && args[1] == "--file" {
        if let Ok(records) = read_local_file(&args[2], filter_category, query) {
            if records.is_empty() {
                println!("Could not find film!");
            } else {
                for r in &records {
                    println!("{r}");
                }
            }
        }
    } else {
        let dataset_filename = OsString::from(DATASETS[input.0].to_string() + ".gz");
        if let Ok(records) = download_file(dataset_filename, filter_category, query).await {
            records
                .first()
                .as_ref()
                .map_or_else(|| println!("No record found!"), |r| println!("{r}"));
        }
    }
    Ok(())
}

fn menu() -> (usize, String) {
    CATEGORIES
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

    let input = match &input {
        Ok(i) => i.trim(),
        Err(_e) => "",
    };
    let input = input.parse::<usize>().unwrap_or(0);
    print!("\nEnter a {}: ", CATEGORIES[input - 1]);
    let search_term = {
        let mut input_buf = String::new();
        let bytes = match stdin().read_line(&mut input_buf) {
            Ok(_) => Ok(input_buf),
            Err(e) => Err(e),
        };
        bytes.map_or_else(|_| todo!(), |i| i)
    };
    (input, search_term)
}

fn read_local_file(
    file_path: &OsString,
    filter_category: &str,
    query: &str,
) -> Result<Vec<Box<dyn Record>>> {
    let file = File::open(file_path)?;
    let file_path = Path::new(file_path)
        .file_name()
        .map_or(Some(""), OsStr::to_str)
        .unwrap_or("");
    let filter = FILTERS.get(filter_category);

    Ok(get_records::parse_records_to_vec(
        BufReader::new(file),
        file_path,
        filter,
        query,
    ))
}

async fn download_file(
    dataset_url: OsString,
    filter_category: &str,
    query: &str,
) -> Result<Vec<Box<dyn Record>>> {
    let dataset_url = dataset_url.to_str().map_or("", |d| d);
    let dataset = get_records::download_dataset(dataset_url).await;
    let file = dataset?.path().join(dataset_url);
    println!("Downloaded file to {}, now unzipping...", file.display());
    let bytes = get_records::decompress_content(&file)?;
    let decomp_file = Path::new(&file);
    write(decomp_file, bytes)?;
    let decomp_file = File::open(decomp_file)?;
    let filter = FILTERS.get(filter_category);
    Ok(get_records::parse_records_to_vec(
        BufReader::new(decomp_file),
        dataset_url,
        filter,
        query,
    ))
}
