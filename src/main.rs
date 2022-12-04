use std::io::stdin;
use anyhow::Result;

mod download;

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
    let temp_dir = download::download_films(&filename).await?;
    let file = temp_dir.path().join(&filename);
    println!("{}", file.display());
    let file = download::decompress_content(&file)?;
    download::get_records_from_file(&file);
    Ok(())
}

