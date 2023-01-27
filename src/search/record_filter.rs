use std::collections::HashMap;

use crate::record_structs::record::Record;
use lazy_static::lazy_static;

pub type FilterFunc = fn(lhs: &Box<dyn Record>) -> bool;

lazy_static! {
    pub static ref FILTERS: HashMap<&'static str, FilterFunc> = {
        let mut map: HashMap<&str, FilterFunc> = HashMap::new();
        map.insert("Title", |_r| true);
        map.insert("Director", |_r| true);
        map.insert("Cast", |_r| true);
        map.insert("Actor", |_r| true);
        map
    };
}
