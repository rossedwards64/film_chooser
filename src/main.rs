use crate::record_structs::record::Record;
use anyhow::Result;
use std::{
    env::args,
    fs::File,
    io::{stdin, Read},
};

mod get_records;
mod record_structs;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let options: [&str; 6] = [
        "Title",
        "Director",
        "Rating",
        "Language",
        "Genre",
        "Release Date",
    ];

    let datasets: [&str; 6] = [
        //        "title.akas.tsv.gz", // because this is just alternate names for films and shows,
        //                                it might work better as a way to view lists of films after searching them
        "name.basics.tsv.gz",
        "title.basics.tsv.gz",
        "title.crew.tsv.gz",
        "title.episode.tsv.gz",
        "title.principals.tsv.gz",
        "title.ratings.tsv.gz",
    ];

    println!("How would you like to search for a film?");
    options
        .iter()
        .enumerate()
        .map(|(i, _opt)| (i + 1, _opt))
        .for_each(|(i, opt)| {
            println!("{i}: {opt}");
        });

    let mut input = String::new();
    let input = match stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(e),
    };

    let parse_input: usize = match &input {
        Ok(i) => i.trim().parse::<usize>().unwrap_or(0),
        Err(_e) => 0,
    };

    let query = options[parse_input - 1];
    println!("{query}");

    if !args.is_empty() && args[1] == "--file" {
        read_local_file(&datasets[0].to_string())
    } else {
        download_file(&query.to_string()).await
    }
}

fn read_local_file(filename: &String) -> Result<()> {
    println!("Using local file {filename}");
    if let Ok(mut records) = File::open(filename) {
        let movie_list: Vec<Box<dyn Record>> = {
            let mut bytes: Vec<u8> = Vec::new();
            println!("Getting bytes from file");
            records.read_to_end(&mut bytes)?;
            println!("File read, now getting records");
            get_records::get_records_from_file(&bytes)?
        };
        println!("Acquired records from file");
        println!("{}", movie_list.first().unwrap());
    } else {
        println!("Unable to open file!");
    }
    Ok(())
}

async fn download_file(filename: &String) -> Result<()> {
    if let Ok(temp_dir) = get_records::download_films(filename).await {
        let movie_list: Vec<Box<dyn Record>> = {
            let file = temp_dir.path().join(filename);
            println!("Downloaded file to {}, now unzipping...", file.display());
            let bytes = get_records::decompress_content(&file)?;
            get_records::get_records_from_file(&bytes)?
        };
        println!("Download complete");
        println!("{}", movie_list.first().unwrap());
    } else {
        println!("Unable to download file!");
    };
    Ok(())
}
