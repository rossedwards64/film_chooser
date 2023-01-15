use std::fmt::Display;

pub trait Record: Display {
    fn get_bool(obj_fields: &[String], idx: usize) -> bool
    where
        Self: Sized,
    {
        match &obj_fields.get(idx) {
            Some(b) => &*b.to_string() == "1",
            None => false,
        }
    }

    fn get_field(obj_fields: &[String], idx: usize) -> String
    where
        Self: Sized,
    {
        match obj_fields.get(idx) {
            Some(r) => r.to_string(),
            None => "None".to_owned(),
        }
    }

    fn build(obj_fields: &[String]) -> Box<dyn Record>
    where
        Self: Sized;
}
