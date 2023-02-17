use crate::{
    get_records::{
        files::{common::path_to_string, local_file::parse_records_from_file},
        parse_records,
    },
    record_structs::record::Record,
};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{
    fs::{write, File as StdFile},
    io::Read,
    path::{Path, PathBuf},
};
use tempfile::Builder;
use tokio::{fs::File as AsyncFile, io::copy};

pub async fn run_download<P: AsRef<Path>>(dataset: P, filter_category: &str, query: &str) {
    println!(
        "Unable to find file {}, attempting download instead.",
        dataset.as_ref().display()
    );
    if let Ok(records) = download_and_parse(dataset, filter_category, query).await {
        parse_records::check_records(&records);
    }
}

async fn download_and_parse<P: AsRef<Path>>(
    dataset_url: P,
    filter_category: &str,
    query: &str,
) -> Result<Vec<Box<dyn Record>>> {
    let dataset_url = path_to_string(dataset_url.as_ref());
    let dataset = download_dataset(dataset_url.clone()).await;
    let file = dataset?.as_path().join(dataset_url);
    println!("Downloaded file to {}, now unzipping...", file.display());
    let decomp_file = decompress_content(&file)?;
    parse_records_from_file(decomp_file, filter_category, query)
}

async fn download_dataset(query: String) -> Result<PathBuf> {
    let temp_dir = Builder::new().prefix("tmp_").rand_bytes(5).tempdir()?;
    let query_gz = query + ".gz";
    let request_url = format!("https://datasets.imdbws.com/{query_gz}");
    println!("Downloading film report from {request_url}");
    let response = reqwest::get(&request_url).await?;
    let mut dest = {
        let filename = temp_dir.path().join(
            response
                .url()
                .path_segments()
                .and_then(Iterator::last)
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp"),
        );
        println!("File will be located in '{filename:?}'");
        AsyncFile::create(filename).await?
    };
    let content = response.bytes().await?;
    copy(&mut &*content, &mut dest).await?;
    Ok(temp_dir.path().to_path_buf())
}

fn decompress_content<P: AsRef<Path>>(file: P) -> Result<PathBuf> {
    let mut decoder = GzDecoder::new(StdFile::open(&file)?);
    let mut file_bytes: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut file_bytes)?;
    let mut decomp_file = PathBuf::from(file.as_ref());
    match write(&mut decomp_file, file_bytes) {
        Ok(_) => println!("File decompressed successfully"),
        Err(e) => eprintln!("Failed to decompress file, {e}"),
    }
    Ok(decomp_file)
}
