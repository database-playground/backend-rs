//! Operate with the database and the models.

#[cfg(any(feature = "seeder"))]
pub mod seeder;

use std::borrow::Cow;

use sqlx::{types::time, Executor, Postgres};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub id: String,
    pub picture: Option<String>,
    pub description: String,
    pub created_at: time::PrimitiveDateTime,
    pub updated_at: time::PrimitiveDateTime,
}

pub async fn get_schema(
    conn: impl Executor<'_, Database = Postgres>,
    schema_id: &str,
) -> Result<Schema, Error> {
    sqlx::query_as!(
        Schema,
        r#"
        --sql
        SELECT schema_id AS id, picture, description, created_at, updated_at
        FROM dp_schemas
        WHERE schema_id = $1;
        "#,
        schema_id
    )
    .fetch_one(conn)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "schema",
            id: Cow::Borrowed(schema_id),
        },
        e => Error::DatabaseError(e),
    })
}

pub async fn get_schema_initial_sql(
    conn: impl Executor<'_, Database = Postgres>,
    schema_id: &str,
) -> Result<String, Error> {
    sqlx::query!(
        r#"
        --sql
        SELECT initial_sql
        FROM dp_schemas
        WHERE schema_id = $1;
        "#,
        schema_id
    )
    .fetch_one(conn)
    .await
    .map(|row| row.initial_sql)
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound {
            entity: "schema",
            id: Cow::Borrowed(schema_id),
        },
        e => Error::DatabaseError(e),
    })
}

#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("{entity} not found: {id}")]
    NotFound {
        entity: &'static str,
        id: Cow<'a, str>,
    },
}
