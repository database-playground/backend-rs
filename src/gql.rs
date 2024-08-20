//! GraphQL schemas.

pub mod error;
pub mod questions;
pub mod schema;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(pub schema::SchemaQuery, pub questions::QuestionQuery);
