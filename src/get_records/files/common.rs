use anyhow::Result;
use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

pub fn get_full_path<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    path.as_ref()
        .canonicalize()
        .map_or_else(|_| path.as_ref().to_path_buf(), |p| p)
}

pub fn get_reader_from_path<P>(path: P) -> Result<File, io::Error>
where
    P: AsRef<Path>,
{
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    Ok(file)
}

pub fn path_to_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    path.as_ref().to_str().map_or("", |s| s).to_string()
}
