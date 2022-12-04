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

struct Record {
    id: String,
    title_type: String,
    primary_title: String,
    original_title: String,
    is_adult: String,
    start_year: String,
    end_year: String,
    runtime_minutes: String,
    genres: String /* get field, and parse commas */
}

pub async fn download_films(x: &String) -> Result<TempDir> {
    let temp_dir = Builder::new()
        .prefix("tmp_")
        .rand_bytes(5)
        .tempdir()?;
    let request_url = format!("https://datasets.imdbws.com/{x}");
    println!("Downloading film report from {request_url}");
    let response = reqwest::get(&request_url).await?;
    let mut dest = {
        let filename = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp");
        let filename = temp_dir.path().join(filename);
        println!("File will be located in '{:?}'", filename);
        AsyncFile::create(filename).await?
    };
    let content = response.bytes().await?;
    copy(&mut content.deref(), &mut dest).await?;
    Ok(temp_dir)
}

pub fn decompress_content(file: &PathBuf) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(StdFile::open(file).unwrap());
    let mut v: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut v)?;
    Ok(v)
}

/*  */
pub fn get_records_from_file(file: &Vec<u8>) {
    let reader = BufReader::new(file.as_slice());
    for line in reader.lines() {
        for (i, item) in line.iter().enumerate() {
            println!("{}: {}", i + 1, item);
        }
    }
}
