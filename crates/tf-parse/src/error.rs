use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid or corrupt file.")]
    ParseError {
        #[from]
        source: fitparser::Error,
    },

    #[error("Missing vital information.")]
    MissingData,
}
