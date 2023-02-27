use std::{fmt::Display, string::ToString};

pub trait Record: Display {
    fn get_field_bool(obj_fields: &[String], idx: usize) -> bool
    where
        Self: Sized,
    {
        obj_fields
            .get(idx)
            .as_ref()
            .map_or(false, |b| &*(*b).to_string() == "1")
    }

    fn get_field_vec(obj_fields: &[String], idx: usize) -> Vec<String>
    where
        Self: Sized,
    {
        Self::get_field(obj_fields, idx)
            .split_terminator(' ')
            .map(ToString::to_string)
            .collect()
    }

    fn get_field_int(obj_fields: &[String], idx: usize) -> u32
    where
        Self: Sized,
    {
        Self::get_field(obj_fields, idx).parse::<u32>().unwrap_or(0)
    }

    fn get_field_float(obj_fields: &[String], idx: usize) -> f32
    where
        Self: Sized,
    {
        Self::get_field(obj_fields, idx)
            .parse::<f32>()
            .unwrap_or(0.0)
    }

    fn get_field(obj_fields: &[String], idx: usize) -> String
    where
        Self: Sized,
    {
        obj_fields
            .get(idx)
            .map_or_else(|| "None".to_owned(), ToString::to_string)
    }

    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized;
}
