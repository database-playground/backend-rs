//! Schema-related database operations.

use super::Error;
use chrono::Utc;
use sqlx::{Executor, Postgres};

#[derive(Debug, Clone)]
pub struct Schema {
    pub schema_id: String,
    pub picture: Option<String>,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[tracing::instrument(skip(conn))]
pub async fn get_schema(
    conn: impl Executor<'_, Database = Postgres>,
    schema_id: &str,
) -> Result<Schema, Error> {
    sqlx::query_as!(
        Schema,
        r#"
        SELECT schema_id, picture, description, created_at, updated_at
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
            id: schema_id.into(),
        },
        e => Error::DatabaseError(e),
    })
}

#[tracing::instrument(skip(conn))]
pub async fn get_schema_initial_sql(
    conn: impl Executor<'_, Database = Postgres>,
    schema_id: &str,
) -> Result<String, Error> {
    sqlx::query!(
        r#"
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
            id: schema_id.into(),
        },
        e => Error::DatabaseError(e),
    })
}
