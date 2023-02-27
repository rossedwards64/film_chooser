use crate::{
    get_records::{
        files::{common::path_to_string, local_file::parse_records_from_file},
        parse_records,
    },
    get_user_input,
    record_structs::record::Record,
};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{
    fs::write,
    io::Read,
    path::{Path, PathBuf},
};
use tempfile::{Builder, TempDir};

enum DownloadDir<'a> {
    Dir(PathBuf),
    TempDir(&'a TempDir),
}

pub async fn run_download<P: AsRef<Path> + Send>(dataset: P, filter_category: &str, query: &str) {
    println!(
        "Unable to find file {}, attempting download instead.",
        dataset.as_ref().display()
    );
    if let Ok(records) = download_and_parse(dataset, filter_category, query).await {
        parse_records::check_records(&records);
    }
}

async fn download_and_parse<P: AsRef<Path> + Send>(
    dataset_url: P,
    filter_category: &str,
    query: &str,
) -> Result<Vec<Box<dyn Record>>> {
    let dataset_url = path_to_string(dataset_url.as_ref());
    let dataset = download_dataset(dataset_url.clone()).await?;
    let file = match ask_to_persist(&dataset) {
        DownloadDir::Dir(p) => p.join(dataset_url),
        DownloadDir::TempDir(t) => t.path().join(dataset_url),
    };

    println!("Downloaded file to {}, now unzipping...", file.display());
    let decomp_file = decompress_content(&file)?;
    parse_records_from_file(decomp_file, filter_category, query)
}

async fn download_dataset(query: String) -> Result<TempDir> {
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
        tokio::fs::File::create(filename).await?
    };

    let content = response.bytes().await?;
    tokio::io::copy(&mut &*content, &mut dest).await?;
    Ok(temp_dir)
}

fn decompress_content<P: AsRef<Path>>(file: P) -> Result<PathBuf> {
    let mut decoder = GzDecoder::new(std::fs::File::open(&file)?);
    let mut file_bytes: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut file_bytes)?;
    let mut decomp_file = PathBuf::from(file.as_ref());
    match write(&mut decomp_file, file_bytes) {
        Ok(_) => println!("File decompressed successfully"),
        Err(e) => eprintln!("Failed to decompress file, {e}"),
    }
    Ok(decomp_file)
}

fn ask_to_persist(temp_dir: &TempDir) -> DownloadDir {
    let mut valid_input = false;
    while !valid_input {
        println!("Would you like to permanently save this dataset? y/n");
        let mut input_buf = get_user_input();
        if input_buf.eq_ignore_ascii_case("y") {
            println!("Saving dataset...");
            let new_path = Path::new("/home/ross/Documents/programming/rust/film_chooser/datasets");
            match std::fs::copy(temp_dir.path(), new_path) {
                Ok(s) => println!("Copied {s} bytes to {}", new_path.display()),
                Err(e) => eprintln!("{e}"),
            };
            return DownloadDir::Dir(new_path.to_path_buf());
        } else if input_buf.eq_ignore_ascii_case("n") {
            println!("Dataset directory will be disposed of when the program ends");
            valid_input = true;
        } else {
            println!("Answer either y/n!");
            input_buf.clear();
        }
    }
    return DownloadDir::TempDir(temp_dir);
}
