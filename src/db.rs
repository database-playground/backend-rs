//! Operate with the database and the models.

#[cfg(feature = "seeder")]
pub mod seeder;

pub mod cursor;
pub use cursor::Cursor;
pub mod user;
pub use user::*;
pub mod event;
pub use event::*;
pub mod question;
pub use question::*;
pub mod schema;
pub use schema::*;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub trait Executor<'a>: sqlx::Executor<'a, Database = sqlx::Postgres> + Copy {}

impl<'a, E> Executor<'a> for E where E: sqlx::Executor<'a, Database = sqlx::Postgres> + Copy {}

/// Create a database connection pool from the DATABASE_URL environment variable.
pub async fn pool() -> Result<Pool, Error> {
    let db_url = std::env::var("DATABASE_URL").map_err(|_| Error::MissingDatabaseUrl)?;
    Ok(sqlx::postgres::PgPoolOptions::new()
        .connect(&db_url)
        .await?)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing DATABASE_URL environment variable")]
    MissingDatabaseUrl,

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{entity} not found: {id}")]
    NotFound {
        entity: &'static str,
        id: ecow::EcoString,
    },

    #[error("id must be a positive integer")]
    NotPositiveID,
}
