use std::{fs::File as StdFile,
          io::{stdin, BufReader, BufRead, Read},
          path::PathBuf, ops::Deref};
use tokio::{io::copy, fs::File};
use error_chain::error_chain;
use tempfile::{Builder, TempDir};
use flate2::read::GzDecoder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
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

    let filename = String::from("title.basics.tsv.gz");
    let temp_dir = download_films(&filename).await?;
    let file = temp_dir.path().join(&filename);
    println!("{}", file.display());
    let bytes = decompress_content(&file)?;
    print_films(&bytes);

    Ok(())
}

async fn download_films(x: &String) -> Result<TempDir> {
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
        File::create(filename).await?
    };
    let content = response.bytes().await?;
    copy(&mut content.deref(), &mut dest).await?;
    Ok(temp_dir)
}

fn decompress_content(file: &PathBuf) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(StdFile::open(file).unwrap());
    let mut v: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut v)?;
    Ok(v)
}

fn print_films(bytes: &Vec<u8>) {
    let reader = BufReader::new(bytes.as_slice());
    for line in reader.lines() {
        match line {
            Ok(c) => println!("{}", c.as_str()),
            Err(err) => {
                println!("{err}");
                break;
            },
        }
    }
}
