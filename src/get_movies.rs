use std::{ops::Deref,
          path::PathBuf,
          fs::File as StdFile,
          io::{Read,
               BufReader,
               BufRead}};
use tokio::{io::copy,
            fs::File as AsyncFile};
use tempfile::{Builder,
               TempDir};
use flate2::read::GzDecoder;
use anyhow::Result;

pub mod record;
use crate::get_movies::record::Record;


pub async fn download_films(x: &String) -> Result<TempDir> {
    let temp_dir = Builder::new()
        .prefix("tmp_")
        .rand_bytes(5)
        .tempdir()?;
    let request_url = format!("https://datasets.imdbws.com/{x}");
    println!("Downloading film report from {request_url}");
    let response = reqwest::get(&request_url).await?;
    let mut dest = {
        let filename = temp_dir
            .path()
            .join(response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp"));
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

pub fn get_records_from_file(file: &Vec<u8>) -> Result<Vec<Record>> {
    Ok(BufReader::new(file.as_slice())
       .lines()
       .map(|line|
            match line {
                Ok(l) => l,
                Err(err) => err.to_string()})
       .filter(|r| { !is_header(r) })
       .map(|line| -> Record {
        record::build_record(&String::from(line)
                             .split_terminator('\t')
                             .map(|f| { f.to_string() })
                             .collect())
       }).collect())
}

fn is_header(input: &String) -> bool {
    return matches!(input.as_ref(), "tconst\ttitleType\tprimaryTitle\toriginalTitle\tisAdult\tstartYear\tendYear\truntimeMinutes\tgenres")
}
