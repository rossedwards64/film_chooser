use crate::{
    get_records::files::{common, download_file, local_file},
    record_structs::dataset_map,
};
use anyhow::Result;
use std::{env::args, io::stdin};

mod get_records;
mod record_structs;
mod search;

#[tokio::main]
async fn main() -> Result<()> {
    let dataset_dir = {
        let path = get_args().map_or_else(String::new, |s| s);
        common::get_full_path(path)
    };
    let record_type = get_category();
    println!("Searching by {record_type}");

    match dataset_map::get_dataset_if_exists(&record_type, dataset_dir) {
        Ok(dataset) => {
            println!("Enter a keyword to search records by");
            let query = get_user_input();
            println!("Filtering records with {query}");
            if dataset.exists() {
                local_file::run_local(dataset, &record_type, &query);
            } else {
                download_file::run_download(dataset, &record_type, &query).await;
            }
        }
        Err(err) => {
            eprintln!("Failure when fetching file: {err}");
        }
    }
    Ok(())
}

fn get_category() -> String {
    dataset_map::print_dataset_keys();
    input_loop()
}

fn input_loop() -> String {
    println!("Select a search option: ");
    let mut valid_input = false;
    let mut input = String::new();
    while !valid_input {
        input = get_user_input();
        if dataset_map::is_valid_key(&input) {
            valid_input = true;
        } else {
            println!("Invalid search term! Please select one of the listed options.");
            input.clear();
        }
    }
    input
}

fn get_user_input() -> String {
    let mut input_buf = String::new();
    match stdin().read_line(&mut input_buf) {
        Ok(_) | Err(_) => input_buf.trim().to_string(),
    }
}

fn get_args() -> Option<String> {
    let args: Vec<String> = args().collect();
    if args[1].is_empty() {
        None
    } else {
        Some(args[1].clone())
    }
}
