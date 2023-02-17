use lazy_static::lazy_static;
use std::{collections::BTreeMap, path::Path};

lazy_static! {
    pub static ref DATASETS: BTreeMap<&'static str, &'static Path> = {
        let mut map: BTreeMap<&'static str, &Path> = BTreeMap::new();
        map.insert("titles", Path::new("title.akas.tsv"));
        map.insert("films", Path::new("title.basics.tsv"));
        map.insert("crew", Path::new("title.crew.tsv"));
        map.insert("episodes", Path::new("title.episode.tsv"));
        map.insert("cast", Path::new("title.principals.tsv"));
        map.insert("ratings", Path::new("title.ratings.tsv"));
        map.insert("names", Path::new("names.basics.tsv"));
        map
    };
}

pub fn is_valid_key<S: AsRef<str>>(key: S) -> bool {
    DATASETS.contains_key(key.as_ref().trim())
}
