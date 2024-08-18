//! GraphQL schemas.

pub mod error;
pub mod schema;

use async_graphql::MergedObject;

#[derive(MergedObject)]
pub struct Query(pub schema::SchemaQuery);
