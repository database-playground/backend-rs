use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use ecow::EcoString;

use crate::db;

pub enum ErrorCode {
    NotFound,
    InternalError,
}

pub struct Error {
    pub code: ErrorCode,
    pub title: EcoString,
    pub details: Cow<'static, str>,
    pub error: Box<dyn std::error::Error + Send + Sync>,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.details)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {}\n{}",
            self.code, self.title, self.details, self.error
        )
    }
}

impl From<db::Error> for Error {
    fn from(value: db::Error) -> Self {
        match value {
            db::Error::NotFound { entity, ref id } => Self {
                code: ErrorCode::NotFound,
                title: EcoString::inline("No resource"),
                details: Cow::Owned(format!("{entity} with id {id} not found")),
                error: Box::new(value),
            },
            e => Self {
                code: ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("An internal error occurred"),
                error: Box::new(e),
            },
        }
    }
}
