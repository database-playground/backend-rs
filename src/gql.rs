//! GraphQL schemas.

pub mod error;
pub mod questions;
pub mod schema;
pub mod sql_executor;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(pub schema::SchemaQuery, pub questions::QuestionQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(pub sql_executor::SqlExecutorMutation);
