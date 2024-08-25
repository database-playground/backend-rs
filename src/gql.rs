//! GraphQL schemas.

pub mod auth;
pub mod error;
pub mod poem;
pub mod questions;
pub mod schema;
pub mod sql_executor;
pub mod user;

use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(
    pub schema::SchemaQuery,
    pub questions::QuestionQuery,
    pub user::UserQuery,
);

#[derive(MergedObject, Default)]
pub struct Mutation(pub sql_executor::SqlExecutorMutation);
