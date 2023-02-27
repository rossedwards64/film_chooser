use crate::get_records::files::common;
use lazy_static::lazy_static;
use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
};

lazy_static! {
    // using a BTreeMap so the keys can be sorted for printing
    static ref DATASETS: BTreeMap<&'static str, &'static Path> = {
        BTreeMap::from(
            [("titles", Path::new("title.akas.tsv")),
             ("films", Path::new("title.basics.tsv")),
             ("crew", Path::new("title.crew.tsv")),
             ("episodes", Path::new("title.episode.tsv")),
             ("cast", Path::new("title.principals.tsv")),
             ("ratings", Path::new("title.ratings.tsv")),
             ("names", Path::new("names.basics.tsv"))]
        )
    };
}

pub fn is_valid_key<S>(key: S) -> bool
where
    S: AsRef<str>,
{
    DATASETS.contains_key(key.as_ref().trim())
}

pub fn print_dataset_keys() {
    DATASETS
        .keys()
        .enumerate()
        .map(|(idx, key)| (idx + 1, key))
        .for_each(|(i, opt)| {
            println!("{i}: {opt}");
        });
}

pub fn get_dataset(dataset_key: &str) -> Result<PathBuf, io::ErrorKind> {
    DATASETS
        .get(dataset_key)
        .map_or(Err(io::ErrorKind::NotFound), |dataset| {
            Ok(PathBuf::from(dataset))
        })
}

pub fn get_dataset_if_exists<P>(dataset_key: &str, dataset_dir: P) -> Result<PathBuf, io::ErrorKind>
where
    P: AsRef<Path>,
{
    DATASETS
        .get(dataset_key)
        .map_or(Err(io::ErrorKind::NotFound), |dataset| {
            Ok(dataset_dir.as_ref().join(dataset))
        })
}
