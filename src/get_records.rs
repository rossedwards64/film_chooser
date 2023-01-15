use crate::record_structs::{
    record::Record,
    film::Film};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{
    fs::File as StdFile,
    io::{BufRead, BufReader, Read},
    ops::Deref,
    path::PathBuf,
};
use tempfile::{Builder, TempDir};
use tokio::{fs::File as AsyncFile, io::copy};

pub async fn download_films(query: &String) -> Result<TempDir> {
    let temp_dir = Builder::new().prefix("tmp_").rand_bytes(5).tempdir()?;
    let request_url = format!("https://datasets.imdbws.com/{query}");
    println!("Downloading film report from {request_url}");
    let response = reqwest::get(&request_url).await?;
    let mut dest = {
        let filename = temp_dir.path().join(
            response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp"),
        );
        println!("File will be located in '{:?}'", filename);
        AsyncFile::create(filename).await?
    };
    let content = response.bytes().await?;
    copy(&mut content.deref(), &mut dest).await?;
    Ok(temp_dir)
}

pub fn decompress_content(file: &PathBuf) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(StdFile::open(file)?);
    let mut v: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut v)?;
    Ok(v)
}

pub fn get_records_from_file(file: &Vec<u8>) -> Result<Vec<Box<dyn Record>>> {
    Ok(BufReader::new(file.as_slice())
        .lines()
        .map(|line| match line {
            Ok(l) => l,
            Err(err) => err.to_string(),
        })
        .filter(|l| !is_header(l))
        .map(|field| {
            let record_fields: Vec<String> = field
                .split_terminator('\t')
                .map(|f| f.to_string())
                .collect();
            Film::build(&record_fields)
        })
        .collect())
}

fn is_header(input: &String) -> bool {
    matches!(input.as_ref(), "tconst\ttitleType\tprimaryTitle\toriginalTitle\tisAdult\tstartYear\tendYear\truntimeMinutes\tgenres")
}
