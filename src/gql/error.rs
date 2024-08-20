use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

use async_graphql::ErrorExtensions;
use ecow::EcoString;

use crate::db;

pub enum ErrorCode {
    NotFound,
    InternalError,
    Unauthorized,
    InvalidJwtToken, // poem
    InvalidQuery,    // sql_executor
}

pub struct Error {
    pub code: ErrorCode,
    pub title: EcoString,
    pub details: Cow<'static, str>,
    pub error: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            ErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ErrorCode::InvalidJwtToken => write!(f, "INVALID_JWT_TOKEN"),
            ErrorCode::InvalidQuery => write!(f, "INVALID_QUERY"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = if let Some(ref e) = self.error {
            format!("\n{:?}", e)
        } else {
            "no error provided".to_string()
        };

        write!(
            f,
            "[{}] {}: {}\n{}",
            self.code, self.title, self.details, error
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
                error: Some(Box::new(value)),
            },
            e => Self {
                code: ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("An internal error occurred"),
                error: Some(Box::new(e)),
            },
        }
    }
}

impl Error {
    pub fn to_gql_error(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}: {}", self.title, self.details)).extend_with(
            |_, eev: &mut async_graphql::ErrorExtensionValues| {
                eev.set("code", self.code.to_string());
                eev.set("title", self.title.as_str());
                eev.set("details", self.details.as_ref());
            },
        )
    }
}

pub fn gqlize<E: Into<Error>>(e: E) -> async_graphql::Error {
    let e: Error = e.into();
    e.to_gql_error()
}
