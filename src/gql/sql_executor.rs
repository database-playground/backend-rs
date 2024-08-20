use std::borrow::Cow;

use crate::{
    db,
    gql::error,
    rpc::{
        self,
        dbrunner::{run_query_response::ResponseType, RunQueryRequest},
    },
};
use async_graphql::{Context, Object, Result, SimpleObject, Union};
use ecow::EcoString;

#[derive(Default)]
pub struct SqlExecutorMutation;

#[Object]
impl SqlExecutorMutation {
    pub async fn execute<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        question_id: i64,
        sql: String,
    ) -> Result<ExecuteResult> {
        let pool = ctx.data::<db::Pool>()?;
        let mut dbrunner = ctx.data::<rpc::DbRunnerClient>()?.clone();

        let initial_sql = db::get_question_schema_initial_sql(pool, question_id)
            .await
            .map_err(error::gqlize)?;

        // run the initial SQL
        let result = dbrunner
            .run_query(RunQueryRequest {
                schema: initial_sql.clone(),
                query: sql,
            })
            .await
            .map_err(|e| match e {
                _ if e.code() == tonic::Code::InvalidArgument => error::Error {
                    code: error::ErrorCode::InvalidQuery,
                    title: EcoString::inline("Invalid query"),
                    details: e.message().to_string().into(),
                    error: Some(Box::new(e)),
                },
                _ => error::Error {
                    code: error::ErrorCode::InvalidQuery,
                    title: EcoString::inline("Internal error"),
                    details: Cow::Borrowed("Unable to run your query."),
                    error: Some(Box::new(e)),
                },
            })?;

        match result.into_inner().response_type {
            Some(ResponseType::Id(user_query_id)) => {
                Ok(ExecuteResult::Success(ExecuteSuccessResult {
                    initial_sql,
                    user_query_id,
                    success: true,
                }))
            }
            Some(ResponseType::Error(error)) => {
                Ok(ExecuteResult::Failed(ExecuteFailedResult { error }))
            }
            None => Err(error::Error {
                code: error::ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("Unknown response type."),
                error: None,
            }
            .into()),
        }
    }
}

#[derive(Union)]
pub enum ExecuteResult {
    Success(ExecuteSuccessResult),
    Failed(ExecuteFailedResult),
}

#[derive(SimpleObject)]
pub struct ExecuteSuccessResult {
    #[graphql(visible = false)]
    initial_sql: String,
    #[graphql(visible = false)]
    user_query_id: String,

    pub success: bool,
}

#[derive(SimpleObject)]
pub struct ExecuteFailedResult {
    pub error: String,
}
