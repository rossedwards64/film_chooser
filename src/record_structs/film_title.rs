use crate::record_structs::record::Record;
use std::{
    fmt::{Display, Formatter, Result},
    string::ToString,
};

enum TitleType {
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
            Self::Alternative(s)
            | Self::Dvd(s)
            | Self::Festival(s)
            | Self::Video(s)
            | Self::Tv(s)
            | Self::Working(s)
            | Self::Original(s)
            | Self::ImdbDisplay(s) => s.to_string(),
            Self::None(_) => "N/A".to_string(),
        }
    }
}

// title.akas.tsv.gz
#[derive(Default)]
pub struct FilmTitle {
    tconst: String,
    ordering: u32,
    title: String,
    region: String,
    language: String,
    types: Vec<TitleType>,
    attributes: Vec<String>,
    is_original_title: bool,
}

impl FilmTitle {
    fn get_types_as_strings(&self) -> Vec<String> {
        self.types.iter().map(ToString::to_string).collect()
    }
}

impl Display for FilmTitle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ID: {}\nOrdering: {}\nTitle: {}\nRegion: {}\nLanguage: {}\nTypes:{}\nAttributes: {}\nIs Original Title: {}",
               self.tconst, self.ordering, self.title, self.region, self.language,
               self.get_types_as_strings().join(", "),
               self.attributes.join(", "), self.is_original_title,)
    }
}

impl Record for FilmTitle
where
    dyn Record: Display,
{
    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized,
    {
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

        Box::new(Self {
            tconst: Self::get_field(obj_fields, 0),
            ordering: Self::get_field_int(obj_fields, 1),
            title: Self::get_field(obj_fields, 2),
            region: Self::get_field(obj_fields, 3),
            language: Self::get_field(obj_fields, 4),
            types,
            attributes: Self::get_field_vec(obj_fields, 6),
            is_original_title: Self::get_field_bool(obj_fields, 7),
        })
    }
}
