use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.principals.tsv.gz */
pub struct Cast {
    title_id: String,
    ordering: u64,
    person_id: String,
    category: String,
    job: String,
    characters: String,
}

impl Display for Cast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Record for Cast
where
    dyn Record: Display,
{
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        Box::new(Cast {
            title_id: todo!(),
            ordering: todo!(),
            person_id: todo!(),
            category: todo!(),
            job: todo!(),
            characters: todo!(),
        })
    }
}
