use crate::{record_structs::record::Record, search::record_filter::FilterFunc};

pub fn filter_records(records: &[Box<dyn Record>], search_term: FilterFunc) -> Vec<&dyn Record> {
    records
        .iter()
        .filter(|r| search_term(r))
        .map(|r| &**r)
        .collect()
}
