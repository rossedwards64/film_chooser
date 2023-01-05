use std::{io::{stdin,
               Read},
          env::args,
          fs::File};
use anyhow::Result;

mod get_movies;
use crate::get_movies::record::Record;


#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    println!("How would you like to search for a film?
              1. Title
              2. Director
              3. Rating
              4. Language
              5. Runtime
              6. Genre
              7. Release Date");

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_c) => println!("Searching by {input}"),
        Err(error) => println!("Didn't receive correct option. {error}"),
    }

    if !args.is_empty() && args[1] == "--file" {
        let filename = &args[2];
        println!("Using local file {}", filename);
        if let Ok(mut file) = File::open(filename) {
            let movie_list: Vec<Record> = {
                let mut bytes = Vec::new();
                println!("Getting bytes from file");
                file.read_to_end(&mut bytes)?;
                println!("File read, now getting records");
                get_movies::get_records_from_file(&bytes)?
            };
            println!("Acquired records from file");
            // movie_list.iter().for_each(|m| println!("{}", m));
            println!("{}", movie_list.first().unwrap());
        } else {
            println!("Unable to open file!");
        };
    } else {
        let filename = String::from("title.basics.tsv.gz");
        if let Ok(temp_dir) = get_movies::download_films(&filename).await {
            let movie_list: Vec<Record> = {
                let file = temp_dir.path().join(&filename);
                println!("Downloaded file to {}, now unzipping...", file.display());
                let bytes = get_movies::decompress_content(&file)?;
                get_movies::get_records_from_file(&bytes)?
            };
            movie_list.iter().for_each(|m| println!("{}", m));
        } else {
            println!("Unable to create temporary directory!");
        };
    }
    Ok(())
}

