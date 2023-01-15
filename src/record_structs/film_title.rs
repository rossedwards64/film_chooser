// title.akas.tsv.gz
pub struct FilmTitle {
    id: String,
    ordering: i64,
    title: String,
    region: String,
    language: String,
    types: Vec<TitleTypes>,
    attributes: Vec<String>,
    is_original_title: bool,
}

pub enum TitleTypes {
    Alternative(String),
    Dvd(String),
    Festival(String),
    Tv(String),
    Working(String),
    Original(String),
    ImdbDisplay(String),
}
