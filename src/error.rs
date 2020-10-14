use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("There was a problem with the connection to the database.")]
    SledError {
        #[from]
        source: sled::Error,
    },
    #[error("Parsing fit file failed.")]
    FitError {
        #[from]
        source: fitparser::Error,
    },
    #[error("Failed to deserialize from database.")]
    BincodeError {
        #[from]
        source: std::boxed::Box<bincode::ErrorKind>,
    },
    #[error("{0}")]
    BadServerResponse(&'static str),
    #[error("{0}")]
    BadConfig(&'static str),
    #[error("{0}")]
    /// Don't create this directly. Use Error::bad_database instead.
    BadDatabase(&'static str),
}

impl Error {
    pub fn bad_database(message: &'static str) -> Self {
        panic!("BadDatabase: {}", message);
        Self::BadDatabase(message)
    }
}

