use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.ratings.tsv.gz */
pub struct Rating {
    id: String,
    average_rating: i8,
    num_votes: u64,
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Record for Rating
where
    dyn Record: Display,
{
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        Box::new(Rating {
            id: todo!(),
            average_rating: todo!(),
            num_votes: todo!(),
        })
    }
}
