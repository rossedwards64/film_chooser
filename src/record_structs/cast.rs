use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.principals.tsv.gz */
pub struct Cast {
    title_id: String,
    ordering: u32,
    person_id: String,
    category: String,
    job: String,
    character: String,
}

impl Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title ID: {}\nOrder: {}\nPerson ID: {}\nCategroy: {}\nJob: {}\nCharacter: {}\n",
            self.title_id, self.ordering, self.person_id, self.category, self.job, self.character
        )
    }
}

impl Record for Cast
where
    dyn Record: Display,
{
    fn new(obj_fields: &[String]) -> Box<Cast>
    where
        Self: Sized,
    {
        Box::new(Self {
            title_id: Self::get_field(obj_fields, 0),
            ordering: Self::get_field_num(obj_fields, 1),
            person_id: Self::get_field(obj_fields, 2),
            category: Self::get_field(obj_fields, 3),
            job: Self::get_field(obj_fields, 4),
            character: Self::get_field(obj_fields, 5),
        })
    }
}
