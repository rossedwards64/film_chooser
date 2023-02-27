use super::files::common;
use crate::{
    record_structs::{
        cast::Cast, crew::Crew, episode::Episode, film::Film, film_title::FilmTitle,
        person::Person, rating::Rating, record::Record,
    },
    search::record_filter::TitleFilter,
};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    string::ToString,
};

pub fn collect_records<P>(
    mut file: BufReader<File>,
    dataset: P,
    record_filter: Option<&TitleFilter>,
    query: &str,
) -> Vec<Box<dyn Record>>
where
    P: AsRef<Path>,
{
    println!("Reading file {}...", dataset.as_ref().display());
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
        .filter_map(|line| map_fields_to_record(line, dataset.as_ref()))
        .collect()
}

fn map_fields_to_record<P>(line: &str, dataset: P) -> Option<Box<dyn Record>>
where
    P: AsRef<Path>,
{
    let record_fields: Vec<String> = line
        .split_terminator('\t')
        .map(ToString::to_string)
        .collect();
    let dataset = common::path_to_string(dataset);
    match dataset.as_str() {
        "title.akas.tsv" => Some(FilmTitle::new(&record_fields)),
        "title.basics.tsv" => Some(Film::new(&record_fields)),
        "title.crew.tsv" => Some(Crew::new(&record_fields)),
        "title.episode.tsv" => Some(Episode::new(&record_fields)),
        "title.principals.tsv" => Some(Cast::new(&record_fields)),
        "title.ratings.tsv" => Some(Rating::new(&record_fields)),
        "name.basics.tsv" => Some(Person::new(&record_fields)),
        _ => None,
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
