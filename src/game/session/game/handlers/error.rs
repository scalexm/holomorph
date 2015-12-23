use diesel::*;

pub enum Error {
    Sql(result::Error),
    Other,
}

impl From<result::Error> for Error {
    fn from(err: result::Error) -> Error {
        Error::Sql(err)
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
