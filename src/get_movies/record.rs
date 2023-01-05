use core::fmt::Display;


pub struct Record {
    id: String,
    title_type: String,
    primary_title: String,
    original_title: String,
    is_adult: bool,
    start_year: String,
    end_year: String,
    runtime_minutes: String,
    genres: Vec<String>
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ID: {}\nType: {}\nPrimary Title: {}\nOriginal Title: {}\nAdult: {}\nStart Year: {}\nEnd Year: {}\nRuntime: {}\nGenres: {}",
               self.id, self.title_type, self.primary_title, self.original_title, self.is_adult,
               self.start_year, self.end_year, self.runtime_minutes, self.genres.join(", "))
    }
}

pub fn get_field(record_fields: &Vec<String>, idx: usize) -> String {
    match record_fields.get(idx) {
        Some(r) => r.to_string(),
        None => "None".to_owned(),
    }
}

pub fn build_record(record_fields: &Vec<String>) -> Record {
    let is_adult = match record_fields.get(4) {
        Some(b) => if b.to_string() == "1" { true } else { false }
        None => false,
    };

    let genres: Vec<String> = get_field(record_fields, 8)
        .split_terminator(',')
        .map(|s| { s.to_string() })
        .collect();

    Record {
        id: get_field(record_fields, 0),
        title_type: get_field(record_fields, 1),
        primary_title: get_field(record_fields, 2),
        original_title: get_field(record_fields, 3),
        is_adult,
        start_year: get_field(record_fields, 5),
        end_year: get_field(record_fields, 6),
        runtime_minutes: get_field(record_fields, 7),
        genres
    }
}
