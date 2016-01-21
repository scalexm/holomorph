use diesel::*;
use std::error::Error as StdError;

pub enum Error {
    Sql(result::Error),
    Other,
}

impl From<result::Error> for Error {
    fn from(err: result::Error) -> Error {
        Error::Sql(err)
    }
}

impl From<result::ConnectionError> for Error {
    fn from(err: result::ConnectionError) -> Error {
        Error::Sql(result::Error::DatabaseError(err.description().to_string()))
    }
}

impl From<result::TransactionError<Error>> for Error {
    fn from(err: result::TransactionError<Error>) -> Error {
        match err {
            TransactionError::UserReturnedError(err) => err,
            TransactionError::CouldntCreateTransaction(err) => Error::Sql(err),
        }
    }
}
