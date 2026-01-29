use crate::model;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, serde::Serialize)]
pub enum Error {
    ConfigMissingEnv(&'static str),
    Model(model::Error),
    Message(String),
}

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Self::Model(val)
    }
}

impl From<String> for Error {
    fn from(val: String) -> Self {
        Self::Message(val)
    }
}
