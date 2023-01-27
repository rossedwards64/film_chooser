use crate::{record_structs::record::Record, search::record_filter::FilterFunc};

pub fn search_records<'a>(
    records: &'a [Box<dyn Record>],
    search_term: &FilterFunc,
) -> Vec<&'a dyn Record> {
    records
        .iter()
        .filter(|r| search_term(r))
        .map(|r| &**r)
        .collect()
}
