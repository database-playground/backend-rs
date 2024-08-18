//! GraphQL schemas.

pub mod error;
pub mod schema;

use async_graphql::MergedObject;

#[derive(MergedObject)]
pub struct Query(schema::SchemaQuery);

impl Query {
    pub async fn new_with_pool() -> Result<Self, anyhow::Error> {
        let pool = crate::db::pool().await?;
        Ok(Self(schema::SchemaQuery::new(pool)))
    }
}
