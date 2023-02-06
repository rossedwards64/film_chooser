use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.crew.tsv.gz */
pub struct Crew {
    tconst: String,
    directors: Vec<String>,
    writers: Vec<String>,
}

impl Display for Crew {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nDirector IDs: {}\nWriter IDs: {}\n",
            self.tconst,
            self.directors.join(", "),
            self.writers.join(", ")
        )
    }
}

impl Record for Crew
where
    dyn Record: Display,
{
    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {
            tconst: Self::get_field(obj_fields, 0),
            directors: Self::get_field_vec(obj_fields, 1),
            writers: Self::get_field_vec(obj_fields, 2),
        })
    }
}
