use crate::record_structs::record::Record;
use std::fmt::Display;

/* name.basics.tsv.gz */
pub struct Actor {
    id: String,
    primary_name: String,
    birth_year: u32,
    death_year: u32,
    primary_profession: Vec<String>,
    known_for_titles: Vec<String>,
}

impl Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}\nPrimary Name: {}\nBirth Year: {}\nDeath Year: {}\nPrimary Profession: {}\nKnown for Titles: {}",
        self.id, self.primary_name, self.birth_year, self.death_year, self.primary_profession.join(", "), self.known_for_titles.join(", "))
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
            id: Self::get_field(obj_fields, 0),
            primary_name: Self::get_field(obj_fields, 1),
            birth_year: Self::get_field_num(obj_fields, 2),
            death_year: Self::get_field_num(obj_fields, 3),
            primary_profession: Self::get_field_vec(obj_fields, 4),
            known_for_titles: Self::get_field_vec(obj_fields, 5),
        })
    }
}
