use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.crew.tsv.gz */
pub struct Crew {
    id: String,
    director_ids: Vec<String>,
    writer_ids: Vec<String>,
}

impl Display for Crew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Record for Crew
where
    dyn Record: Display,
{
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        Box::new(Crew {
            id: todo!(),
            director_ids: todo!(),
            writer_ids: todo!(),
        })
    }
}
