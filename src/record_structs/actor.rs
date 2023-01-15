use crate::record_structs::record::Record;
use std::fmt::Display;

pub(crate) enum DeathYear {
    DeathYear(u16),
    None(()),
}

/* name.basics.tsv.gz */
pub struct Actor {
    id: String,
    primary_name: String,
    birth_year: u16,
    death_year: DeathYear,
    primary_profession: Vec<String>,
    known_for_titles: Vec<String>,
}

impl Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Record for Actor
where
    dyn Record: Display,
{
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized,
    {
        Box::new(Actor {
            id: todo!(),
            primary_name: todo!(),
            birth_year: todo!(),
            death_year: todo!(),
            primary_profession: todo!(),
            known_for_titles: todo!(),
        })
    }
}
