use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::db;

use super::error;

pub struct SchemaQuery;

impl SchemaQuery {}

#[Object]
impl SchemaQuery {
    #[tracing::instrument(skip(self, ctx))]
    async fn schema<'ctx>(&self, ctx: &Context<'ctx>, id: String) -> Result<Schema> {
        tracing::debug!("Running GraphQL query 'schema'");
        let pool = ctx.data::<Pool<Postgres>>()?;

        db::get_schema(pool, &id)
            .await
            .map(Into::into)
            .map_err(error::gqlize)
    }
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
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

#[ComplexObject]
impl Schema {
    #[tracing::instrument(skip(self, ctx))]
    async fn initial_sql<'ctx>(&self, ctx: &Context<'ctx>) -> Result<String> {
        tracing::debug!("Running GraphQL query 'schema.initial_sql'");
        let pool = ctx.data::<Pool<Postgres>>()?;

        db::get_schema_initial_sql(pool, &self.id)
            .await
            .map_err(error::gqlize)
    }
}