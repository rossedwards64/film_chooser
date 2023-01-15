use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.episode.tsv.gz */
pub struct Episode {
    id: String,
    parent_id: String,
    season_number: i8,
    episode_number: i16,
}

impl Display for Episode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Record for Episode
where
    dyn Record: Display,
{
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        Box::new(Episode {
            id: todo!(),
            parent_id: todo!(),
            season_number: todo!(),
            episode_number: todo!(),
        })
    }
}
