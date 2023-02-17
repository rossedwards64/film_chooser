use crate::{
    get_records::files::{common, download_file, local_file},
    record_structs::dataset_map::{is_valid_key, DATASETS},
};
use anyhow::Result;
use std::io::stdin;
mod get_records;
mod record_structs;
mod search;

#[tokio::main]
async fn main() -> Result<()> {
    let record_type = get_category().trim().to_string();
    println!("Searching by {record_type}");

    match common::get_dataset_if_exists(&record_type) {
        Ok(dataset) => {
            println!("Successfully acquired dataset.");
            let mut query = String::new();
            println!("Enter a keyword to search records by");
            get_user_input(&mut query);
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
    DATASETS
        .keys()
        .enumerate()
        .map(|(idx, key)| (idx + 1, key))
        .for_each(|(i, opt)| {
            println!("{i}: {opt}");
        });
    println!("Select a search option: ");
    input_loop()
}

fn input_loop() -> String {
    let mut valid_input = false;
    let mut input = String::new();
    while !valid_input {
        get_user_input(&mut input);
        if is_valid_key(&input) {
            valid_input = true;
        } else {
            println!("Invalid search term! Please select one of the listed options.");
            input.clear();
        }
    }
    input
}

fn get_user_input(input_buf: &mut String) -> &String {
    match stdin().read_line(input_buf) {
        Ok(_) | Err(_) => input_buf,
    }
}
