#![deny(rust_2018_idioms)]

use std::error::Error;

#[derive(Debug)]
pub struct ContextualError {
    msg: String,
    error: Box<dyn Error + 'static>,
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.msg, self.error)
    }
}

impl Error for ContextualError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.error)
    }
}

pub trait WithContext<T> {
    fn context<S: Into<String>>(self, msg: S) -> Result<T, ContextualError>;
}

impl<T, E: Error + 'static> WithContext<T> for Result<T, E> {
    fn context<S: Into<String>>(self, msg: S) -> Result<T, ContextualError> {
        self.map_err(|err| ContextualError {
            msg: msg.into(),
            error: Box::new(err),
        })
    }
}
