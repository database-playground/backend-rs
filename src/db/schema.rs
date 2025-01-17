//! Schema-related database operations.

use super::{Error, Executor};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Schema {
    pub schema_id: String,
    pub picture: Option<String>,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[tracing::instrument(skip(conn))]
pub async fn get_schema(conn: impl Executor<'_>, schema_id: &str) -> Result<Schema, Error> {
    tracing::debug!("Getting schema from database");

    sqlx::query_as!(
        Schema,
        r#"
        SELECT schema_id, picture, description, created_at, updated_at
        FROM dp_schemas
        WHERE schema_id = $1 AND deleted_at IS NULL;
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
    conn: impl Executor<'_>,
    schema_id: &str,
) -> Result<String, Error> {
    tracing::debug!("Getting schema initial SQL from database");

    sqlx::query!(
        r#"
        SELECT initial_sql
        FROM dp_schemas
        WHERE schema_id = $1 AND deleted_at IS NULL;
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
