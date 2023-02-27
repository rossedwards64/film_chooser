use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum RecordError {
    #[error("Fields do not map to film title object")]
    FilmTitleError,
    #[error("Fields do not map to film object")]
    FilmError,
    #[error("Fields do not map to crew object")]
    CrewError,
    #[error("Fields do not map to episode object")]
    EpisodeError,
    #[error("Fields do not map to cast object")]
    CastError,
    #[error("Fields do not map to ratings object")]
    RatingError,
    #[error("Fields do not map to person object")]
    PersonError,
}
