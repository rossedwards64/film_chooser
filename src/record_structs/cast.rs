use crate::record_structs::record::Record;
use std::fmt::{Display, Formatter, Result};

/* title.principals.tsv.gz */
#[derive(Default)]
pub struct Cast {
    tconst: String,
    ordering: u32,
    nconst: String,
    category: String,
    job: String,
    characters: String,
}

impl Display for Cast {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Title ID: {}\nOrder: {}\nPerson ID: {}\nCategroy: {}\nJob: {}\nCharacter: {}\n",
            self.tconst, self.ordering, self.nconst, self.category, self.job, self.characters
        )
    }
}

impl Record for Cast
where
    dyn Record: Display,
{
    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {
            tconst: Self::get_field(obj_fields, 0),
            ordering: Self::get_field_int(obj_fields, 1),
            nconst: Self::get_field(obj_fields, 2),
            category: Self::get_field(obj_fields, 3),
            job: Self::get_field(obj_fields, 4),
            characters: Self::get_field(obj_fields, 5),
        })
    }
}
