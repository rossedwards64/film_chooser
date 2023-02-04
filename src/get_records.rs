use crate::record_structs::{
    actor::Actor, cast::Cast, crew::Crew, episode::Episode, film::Film, film_title::FilmTitle,
    rating::Rating, record::Record,
};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{
    fs::File as StdFile,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};
use tempfile::{Builder, TempDir};
use tokio::{fs::File as AsyncFile, io::copy};

pub async fn download_dataset(query: &String) -> Result<TempDir> {
    let temp_dir = Builder::new().prefix("tmp_").rand_bytes(5).tempdir()?;
    let request_url = format!("https://datasets.imdbws.com/{query}");
    println!("Downloading film report from {request_url}");
    let response = reqwest::get(&request_url).await?;
    let mut dest = {
        let filename = temp_dir.path().join(
            response
                .url()
                .path_segments()
                .and_then(std::iter::Iterator::last)
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp"),
        );
        println!("File will be located in '{filename:?}'");
        AsyncFile::create(filename).await?
    };
    let content = response.bytes().await?;
    copy(&mut &*content, &mut dest).await?;
    Ok(temp_dir)
}

pub fn decompress_content(file: &PathBuf) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(StdFile::open(file)?);
    let mut v: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut v)?;
    Ok(v)
}

pub fn parse_records_to_vec(file: BufReader<StdFile>, dataset: &str) -> Vec<Box<dyn Record>> {
    file.lines()
        .map(|line| match line {
            Ok(l) => l,
            Err(err) => err.to_string(),
        })
        .skip(1) // skip header line
        .map(|field| -> Box<dyn Record> {
            let record_fields: Vec<String> = field
                .split_terminator('\t')
                .map(std::string::ToString::to_string)
                .collect();
            match dataset {
                "title.akas.tsv" => FilmTitle::new(&record_fields),
                "title.basics.tsv" => Film::new(&record_fields),
                "title.crew.tsv" => Crew::new(&record_fields),
                "title.episode.tsv" => Episode::new(&record_fields),
                "title.principals.tsv" => Cast::new(&record_fields),
                "title.ratings.tsv" => Rating::new(&record_fields),
                "name.basics.tsv" => Actor::new(&record_fields),
                &_ => todo!(),
            }
        })
        .collect()
}
