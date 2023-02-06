use lazy_static::lazy_static;
use std::collections::HashMap;

// make TitleFilter a struct that stores the search term
pub type TitleFilter = fn(cur_line: &str, input: &str) -> bool;

lazy_static! {
    pub static ref FILTERS: HashMap<&'static str, TitleFilter> = {
        let mut map: HashMap<&str, TitleFilter> = HashMap::new();
        map.insert("Title", |cur_line, input| cur_line.contains(input));
        map.insert("Director", |cur_line, input| true);
        map.insert("Actor", |cur_line, input| true);
        map
    };
}
