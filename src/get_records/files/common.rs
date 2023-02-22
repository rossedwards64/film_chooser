use crate::record_structs::dataset_map::DATASETS;
use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufReader, ErrorKind},
    path::{Path, PathBuf},
};

pub fn get_dataset(dataset_key: &str) -> Result<PathBuf, io::ErrorKind> {
    DATASETS
        .get(dataset_key)
        .map_or(Err(ErrorKind::NotFound), |dataset| {
            Ok(PathBuf::from(dataset))
        })
}

pub fn get_dataset_if_exists<P: AsRef<Path>>(
    dataset_key: &str,
    dataset_dir: P,
) -> Result<PathBuf, io::ErrorKind> {
    DATASETS
        .get(dataset_key)
        .map_or(Err(ErrorKind::NotFound), |dataset| {
            Ok(dataset_dir.as_ref().join(dataset))
        })
}

pub fn get_reader_from_path<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, io::Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    Ok(BufReader::new(file))
}

pub fn path_to_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_str().map_or("", |s| s).to_string()
}
