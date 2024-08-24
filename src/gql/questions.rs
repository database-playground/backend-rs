use async_graphql::{ComplexObject, Context, Enum, Object, Result, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};

use crate::{
    db,
    gql::auth::{ContextAuthExt, Scope},
};

use super::schema::Schema;

#[derive(Default)]
pub struct QuestionQuery;

impl QuestionQuery {}

#[Object]
impl QuestionQuery {
    // FIXME: better questions query (relay connection)

    async fn questions<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Question>> {
        ctx.require_scope(Scope::ReadPublicResource)?;

        tracing::debug!("Running GraphQL query 'questions'");
        let pool = ctx.data::<db::Pool>()?;
        let cursor = db::Cursor { limit, offset };

        db::list_questions(pool, cursor)
            .await
            .map(|questions| questions.into_iter().map(Into::into).collect())
            .map_err(Into::into)
    }

    async fn question<'ctx>(&self, ctx: &Context<'ctx>, id: i64) -> Result<Question> {
        ctx.require_scope(Scope::ReadPublicResource)?;

        tracing::debug!("Running GraphQL query 'question'");
        let pool = ctx.data::<Pool<Postgres>>()?;

        db::get_question(pool, id)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Question {
    pub id: i64,
    pub schema_id: Option<String>,
    pub question_type: String,
    pub difficulty: Difficulty,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Question {
    async fn schema<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<Schema>> {
        ctx.require_scope(Scope::ReadPublicResource)?;

        tracing::debug!("Running GraphQL query 'question.schema'");
        let pool = ctx.data::<Pool<Postgres>>()?;

        match self.schema_id {
            Some(ref schema_id) => db::get_schema(pool, schema_id)
                .await
                .map(|schema| Some(schema.into()))
                .map_err(Into::into),
            None => Ok(None),
        }
    }

    async fn solution<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Option<String>> {
        ctx.require_scope(Scope::ReadSolution)?;

        tracing::debug!("Running GraphQL query 'question.solution'");
        let pool = ctx.data::<Pool<Postgres>>()?;

        db::get_question_solution(pool, self.id)
            .await
            .map_err(Into::into)
    }
}

impl From<db::Question> for Question {
    fn from(question: db::Question) -> Self {
        Self {
            id: question.question_id,
            schema_id: question.schema_id,
            question_type: question.question_type,
            difficulty: question.difficulty.into(),
            title: question.title,
            description: question.description,
            created_at: question.created_at,
            updated_at: question.updated_at,
        }
    }
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl From<db::Difficulty> for Difficulty {
    fn from(difficulty: db::Difficulty) -> Self {
        match difficulty {
            db::Difficulty::Easy => Self::Easy,
            db::Difficulty::Medium => Self::Medium,
            db::Difficulty::Hard => Self::Hard,
        }
    }
}
