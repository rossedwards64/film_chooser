use crate::record_structs::record::Record;
use std::fmt::Display;


/* title.basics.tsv.gz */
pub struct Film {
    id: String,
    title_type: String,
    primary_title: String,
    original_title: String,
    is_adult: bool,
    start_year: String,
    end_year: String,
    runtime_minutes: String,
    genres: Vec<String>,
}

impl Display for Film {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ID: {}\nType: {}\nPrimary Title: {}\nOriginal Title: {}\nAdult: {}\nStart Year: {}\nEnd Year: {}\nRuntime: {}\nGenres: {}",
            self.id, self.title_type, self.primary_title, self.original_title, self.is_adult,
            self.start_year, self.end_year, self.runtime_minutes, self.genres.join(", "))
    }
}

impl Record for Film
where
    dyn Record: Display,
{
    fn get_field(obj_fields: &[String], idx: usize) -> String
    where
        Self: Sized,
    {
        match obj_fields.get(idx) {
            Some(r) => r.to_string(),
            None => "None".to_owned(),
        }
    }

    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        let is_adult = match &obj_fields.get(4) {
            Some(b) => &*b.to_string() == "1",
            None => false,
        };

        let genres: Vec<String> = Self::get_field(obj_fields, 8)
            .split_terminator(',')
            .map(|s| s.to_string())
            .collect();

        Box::new(Film {
            id: Self::get_field(obj_fields, 0),
            title_type: Self::get_field(obj_fields, 1),
            primary_title: Self::get_field(obj_fields, 2),
            original_title: Self::get_field(obj_fields, 3),
            is_adult,
            start_year: Self::get_field(obj_fields, 5),
            end_year: Self::get_field(obj_fields, 6),
            runtime_minutes: Self::get_field(obj_fields, 7),
            genres,
        })
    }
}
