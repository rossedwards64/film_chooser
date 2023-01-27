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

    fn get_field_vec(obj_fields: &[String], idx: usize) -> Vec<String>
    where
        Self: Sized,
    {
        Self::get_field(obj_fields, idx)
            .split_terminator(' ')
            .map(|s| s.to_string())
            .collect()
    }

    fn get_field_num(obj_fields: &[String], idx: usize) -> u32
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
        match obj_fields.get(idx) {
            Some(r) => r.to_string(),
            None => "None".to_owned(),
        }
    }

    fn new(obj_fields: &[String]) -> Box<Self>
    where
        Self: Sized;
}
