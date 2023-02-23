use lazy_static::lazy_static;
use std::{collections::HashMap, io::{BufReader, BufRead}};

use crate::get_records::files::common::{get_dataset, get_reader_from_path};

pub type TitleFilter = fn(cur_line: &str, input: &str) -> bool;

lazy_static! {
    pub static ref FILTERS: HashMap<&'static str, TitleFilter> = {
        let mut map: HashMap<&str, TitleFilter> = HashMap::new();
        map.insert("Title", |cur_line, input| cur_line.contains(input));
        map.insert("Director", |cur_line, input| {
            get_dataset("names").map_or(false, |dataset| {
                get_reader_from_path(dataset).map_or(false, |s| {
                    let s = BufReader::new(s);
                    s.lines()
                        .skip(1)
                        .any(|cur| cur.map_or(false, |c| c.contains(input) && c.contains(cur_line)))
                })
            })
        });
        map.insert("Actor", |cur_line, input| true);
        map
    };
}
