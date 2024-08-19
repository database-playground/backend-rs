//! Operate with the database and the models.

#[cfg(any(feature = "seeder"))]
pub mod seeder;

pub mod cursor;
pub use cursor::Cursor;
pub mod question;
pub use question::{
    get_question, get_question_answer, get_question_solution, list_questions, Difficulty, Question,
};
pub mod schema;
pub use schema::{get_schema, get_schema_initial_sql, Schema};

/// Create a database connection pool from the DATABASE_URL environment variable.
pub async fn pool() -> Result<sqlx::Pool<sqlx::Postgres>, Error> {
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
