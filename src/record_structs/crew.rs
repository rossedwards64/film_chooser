use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.crew.tsv.gz */
pub struct Crew {
    id: String,
    director_ids: Vec<String>,
    writer_ids: Vec<String>,
}

impl Display for Crew {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nDirector IDs: {}\nWriter IDs: {}\n",
            self.id,
            self.director_ids.join(", "),
            self.writer_ids.join(", ")
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
            id: Self::get_field(obj_fields, 0),
            director_ids: Self::get_field_vec(obj_fields, 1),
            writer_ids: Self::get_field_vec(obj_fields, 2),
        })
    }
}
