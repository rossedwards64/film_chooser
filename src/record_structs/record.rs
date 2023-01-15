use std::fmt::Display;


pub trait Record: Display {
    fn get_field(obj_fields: &[String], idx: usize) -> String
    where
        Self: Sized;
    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized;
}
