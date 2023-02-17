use super::common::get_reader_from_path;
use crate::{
    get_records::parse_records, record_structs::record::Record, search::record_filter::FILTERS,
};
use anyhow::Result;
use std::{ffi::OsStr, path::Path};

pub fn run_local<P: AsRef<Path>>(file_path: P, filter_category: &str, query: &str) {
    println!("Found file {}", file_path.as_ref().display());
    if let Ok(records) = parse_records_from_file(&file_path, filter_category, query) {
        parse_records::check_records(&records);
    }
}

pub fn parse_records_from_file<P: AsRef<Path>>(
    file_path: P,
    filter_category: &str,
    query: &str,
) -> Result<Vec<Box<dyn Record>>> {
    let file_path = file_path
        .as_ref()
        .file_name()
        .map_or(Some(""), OsStr::to_str)
        .unwrap_or("");
    let filter = FILTERS.get(filter_category);
    Ok(parse_records::collect_records(
        get_reader_from_path(file_path)?,
        file_path,
        filter,
        query,
    ))
}
