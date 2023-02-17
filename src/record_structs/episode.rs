use crate::record_structs::record::Record;
use std::fmt::Display;

/* title.episode.tsv.gz */
#[derive(Default)]
pub struct Episode {
    tconst: String,
    parent_tconst: String,
    season_number: u32,
    episode_number: u32,
}

impl Display for Episode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nShow ID: {}\nSeason Number: {}\nEpisode Number: {}",
            self.tconst, self.parent_tconst, self.season_number, self.episode_number
        )
    }
}

impl Record for Episode
where
    dyn Record: Display,
{
    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self {
            tconst: Self::get_field(obj_fields, 0),
            parent_tconst: Self::get_field(obj_fields, 1),
            season_number: Self::get_field_num(obj_fields, 4),
            episode_number: Self::get_field_num(obj_fields, 3),
        })
    }
}
