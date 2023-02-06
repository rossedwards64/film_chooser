use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.ratings.tsv.gz */
pub struct Rating {
    tconst: String,
    average_rating: f32,
    num_votes: u32,
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nAverage Rating: {}\nNumber of Votes: {}",
            self.tconst, self.average_rating, self.num_votes
        )
    }
}

impl Record for Rating
where
    dyn Record: Display,
{
    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {
            tconst: Self::get_field(obj_fields, 0),
            average_rating: Self::get_field_float(obj_fields, 1),
            num_votes: Self::get_field_num(obj_fields, 2),
        })
    }
}
