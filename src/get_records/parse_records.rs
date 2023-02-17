use super::files::common::path_to_string;
use crate::{
    record_structs::{
        cast::Cast, crew::Crew, episode::Episode, film::Film, film_title::FilmTitle,
        person::Person, rating::Rating, record::Record,
    },
    search::record_filter::TitleFilter,
};
use std::{
    fs::File as StdFile,
    io::{BufReader, Read},
    path::Path,
};

pub fn collect_records<P: AsRef<Path>>(
    mut file: BufReader<StdFile>,
    dataset: P,
    record_filter: Option<&TitleFilter>,
    query: &str,
) -> Vec<Box<dyn Record>> {
    let no_filter: TitleFilter = |_c, _i| true;
    let record_filter = record_filter.unwrap_or(&no_filter);
    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(size) => println!("Successfully read file. {size} bytes read."),
        Err(e) => println!("Failed to read file to string. {e}"),
    };

    file_str
        .lines()
        .skip(1) // skip header line
        .filter(|c| record_filter(c, query))
        .map(|line| map_fields_to_record(line, dataset.as_ref()))
        .collect()
}

fn map_fields_to_record<P: AsRef<Path>>(line: &str, dataset: P) -> Box<dyn Record> {
    let record_fields: Vec<String> = line
        .split_terminator('\t')
        .map(std::string::ToString::to_string)
        .collect();
    let dataset = path_to_string(dataset);
    match dataset.as_str() {
        "title.akas.tsv" => FilmTitle::new(&record_fields),
        "title.basics.tsv" => Film::new(&record_fields),
        "title.crew.tsv" => Crew::new(&record_fields),
        "title.episode.tsv" => Episode::new(&record_fields),
        "title.principals.tsv" => Cast::new(&record_fields),
        "title.ratings.tsv" => Rating::new(&record_fields),
        "name.basics.tsv" => Person::new(&record_fields),
        _ => unreachable!(),
    }
}

pub fn check_records(records: &[Box<dyn Record>]) {
    if records.is_empty() {
        println!("Could not find any records.");
    } else {
        for r in records {
            println!("{r}");
        }
    }
}
