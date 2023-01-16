use anyhow::Result;
use std::{
    env::args,
    ffi::OsString,
    fs::File,
    io::{stdin, Read},
    path::Path,
};

mod get_records;
mod record_structs;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<OsString> = args().map(OsString::from).collect();
    if !args.is_empty() && args[1] == "--file" {
        read_local_file(&args[2])
    } else {
        let options: [&str; 6] = [
            "Title",
            "Director",
            "Rating",
            "Language",
            "Genre",
            "Release Date",
        ];

        let datasets: [&str; 7] = [
            // because this is just alternate names for films and shows,
            // it might work better as a way to view lists of films after searching them
            "title.akas.tsv",
            "title.basics.tsv",
            "title.crew.tsv",
            "title.episode.tsv",
            "title.principals.tsv",
            "title.ratings.tsv",
            "name.basics.tsv",
        ];

        let input = menu(&options);
        let query = options[input - 1];

        println!("Searching by {query}");

        download_file(OsString::from(datasets[input].to_string() + ".gz")).await
    }
}

fn menu(options: &[&str]) -> usize {
    println!("How would you like to search for a film?");
    options
        .iter()
        .enumerate()
        .map(|(i, _opt)| (i + 1, _opt))
        .for_each(|(i, opt)| {
            println!("{i}: {opt}");
        });

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

fn read_local_file(file_path: &OsString) -> Result<()> {
    println!("Using local file {}", file_path.to_str().expect("N/A"));
    if let Ok(mut records) = File::open(file_path) {
        let movie_list = {
            let mut bytes: Vec<u8> = Vec::new();
            println!("Getting bytes from file");
            records.read_to_end(&mut bytes)?;
            println!("File read, now getting records");
            get_records::get_records_from_file(
                &bytes,
                Path::new(&file_path)
                    .file_name()
                    .expect("Couldn't convert filename to string.")
                    .to_str()
                    .unwrap_or(""),
            )
            .unwrap()
        };
        println!("Acquired records from file");
        println!("{}", movie_list.first().unwrap());
    } else {
        println!("Unable to open file!");
    }
    Ok(())
}

async fn download_file(dataset: OsString) -> Result<()> {
    if let Ok(temp_dir) = get_records::download_films(
        &dataset
            .to_str()
            .expect("Couldnt convert filename to string.")
            .to_string(),
    )
    .await
    {
        let movie_list = {
            let file = temp_dir.path().join(&dataset);
            println!("Downloaded file to {}, now unzipping...", file.display());
            let bytes = get_records::decompress_content(&file)?;
            get_records::get_records_from_file(
                &bytes,
                dataset
                    .to_str()
                    .expect("Couldn't convert filename to string."),
            )?
        };
        println!("Download complete");
        println!("{}", movie_list.first().unwrap());
    } else {
        println!("Unable to download file!");
    };
    Ok(())
}
