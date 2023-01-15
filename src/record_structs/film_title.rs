use crate::record_structs::record::Record;
use std::fmt::Display;

pub(crate) enum TitleType {
    Alternative(String),
    Dvd(String),
    Festival(String),
    Video(String),
    Tv(String),
    Working(String),
    Original(String),
    ImdbDisplay(String),
    None(()),
}

impl ToString for TitleType {
    fn to_string(&self) -> String {
        match self {
            TitleType::Alternative(s)
            | TitleType::Dvd(s)
            | TitleType::Festival(s)
            | TitleType::Video(s)
            | TitleType::Tv(s)
            | TitleType::Working(s)
            | TitleType::Original(s)
            | TitleType::ImdbDisplay(s) => s.to_string(),
            TitleType::None(_) => "N/A".to_string(),
        }
    }
}

// title.akas.tsv.gz
pub struct FilmTitle {
    id: String,
    ordering: i64,
    title: String,
    region: String,
    language: String,
    types: Vec<TitleType>,
    attributes: Vec<String>,
    is_original_title: bool,
}

impl FilmTitle {
    fn get_types_as_strings(&self) -> Vec<String> {
        self.types.iter().map(|t| t.to_string()).collect()
    }
}

impl Display for FilmTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ID: {}\nOrdering: {}\nTitle: {}\nRegion: {}\nLanguage: {}\nTypes:{}\nAttributes: {}\nIs Original Title: {}",
               self.id, self.ordering, self.title, self.region, self.language,
               self.get_types_as_strings().join(", "),
               self.attributes.join(", "), self.is_original_title,)
    }
}

impl Record for FilmTitle
where
    dyn Record: Display,
{
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        let ordering = Self::get_field(obj_fields, 1).parse().unwrap_or(0);
        let types = {
            Self::get_field(obj_fields, 5)
                .split_terminator(' ')
                .map(|t| match t {
                    "alternative" => TitleType::Alternative(t.to_string()),
                    "dvd" => TitleType::Dvd(t.to_string()),
                    "festival" => TitleType::Festival(t.to_string()),
                    "tv" => TitleType::Tv(t.to_string()),
                    "video" => TitleType::Video(t.to_string()),
                    "working" => TitleType::Working(t.to_string()),
                    "original" => TitleType::Original(t.to_string()),
                    "imdbDisplay" => TitleType::ImdbDisplay(t.to_string()),
                    &_ => TitleType::None(()),
                })
                .collect()
        };
        let attributes = Self::get_field(obj_fields, 6)
            .split_terminator(' ')
            .map(|s| s.to_string())
            .collect();
        let is_original_title = Self::get_bool(obj_fields, 7);

        Box::new(FilmTitle {
            id: Self::get_field(obj_fields, 0),
            ordering,
            title: Self::get_field(obj_fields, 2),
            region: Self::get_field(obj_fields, 3),
            language: Self::get_field(obj_fields, 4),
            types,
            attributes,
            is_original_title,
        })
    }
}
