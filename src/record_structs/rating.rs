use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.ratings.tsv.gz */
pub struct Rating {
    id: String,
    average_rating: u32,
    num_votes: u32,
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nAverage Rating: {}\nNumber of Votes: {}",
            self.id, self.average_rating, self.num_votes
        )
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
            id: Self::get_field(obj_fields, 0),
            average_rating: Self::get_field_num(obj_fields, 1),
            num_votes: Self::get_field_num(obj_fields, 2),
        })
    }
}
