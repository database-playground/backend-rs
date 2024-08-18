use async_graphql::{Object, Result, SimpleObject};
use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::db;

use super::error;

pub struct SchemaQuery {
    pub db_pool: Pool<Postgres>,
}

impl SchemaQuery {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { db_pool: pool }
    }
}

#[Object]
impl SchemaQuery {
    #[tracing::instrument(skip(self))]
    async fn schema(&self, id: String) -> Result<Schema> {
        let reply = db::get_schema(&self.db_pool, &id)
            .await
            .map_err(error::Error::from)?;
        Ok(reply.into())
    }
}

#[derive(Debug, SimpleObject)]
pub struct Schema {
    pub id: String,
    pub picture: Option<String>,
    pub description: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl From<db::Schema> for Schema {
    fn from(schema: db::Schema) -> Self {
        Self {
            id: schema.schema_id,
            picture: schema.picture,
            description: schema.description,
            created_at: schema.created_at,
            updated_at: schema.updated_at,
        }
    }
}
