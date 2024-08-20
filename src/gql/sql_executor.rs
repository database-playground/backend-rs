use std::borrow::Cow;

use crate::{
    db,
    gql::{
        auth::{ContextAuthExt, Scope},
        error,
    },
    rpc::{
        self,
        dbrunner::{
            retrieve_query_response::Kind, run_query_response::ResponseType,
            AreQueriesOutputSameRequest, RetrieveQueryRequest, RetrieveQueryResponse,
            RunQueryRequest,
        },
    },
};
use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject, Union};
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
        ctx.require_scope(Scope::Challenge)?;

        let pool = ctx.data::<db::Pool>()?;
        let mut dbrunner = ctx.data::<rpc::DbRunnerClient>()?.clone();

        tracing::debug!(question_id, "Retrieving initial SQL");
        let initial_sql = db::get_question_schema_initial_sql(pool, question_id)
            .await
            .map_err(error::gqlize)?;

        tracing::debug!(initial_sql, sql, "Running user query");
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

        tracing::debug!(question_id, "Constructing response");
        match result.into_inner().response_type {
            Some(ResponseType::Id(user_query_id)) => {
                Ok(ExecuteResult::Success(ExecuteSuccessResult {
                    initial_sql,
                    user_query_id,
                }))
            }
            Some(ResponseType::Error(error)) => {
                Ok(ExecuteResult::Failed(ExecuteFailedResult { error }))
            }
            None => Err(invalid_response_type_error()),
        }
    }
}

#[derive(Union)]
pub enum ExecuteResult {
    Success(ExecuteSuccessResult),
    Failed(ExecuteFailedResult),
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ExecuteSuccessResult {
    #[graphql(visible = false)]
    initial_sql: String,
    #[graphql(visible = false)]
    user_query_id: String,
}

#[ComplexObject]
impl ExecuteSuccessResult {
    async fn rows<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Table> {
        let mut dbrunner = ctx.data::<rpc::DbRunnerClient>()?.clone();

        tracing::debug!(query_id = self.user_query_id, "Retrieving query results");
        let query_response = dbrunner
            .retrieve_query(RetrieveQueryRequest {
                id: self.user_query_id.clone(),
            })
            .await
            .map_err(retrieve_query_internal_error)?;
        let mut query_response_body = query_response.into_inner();

        tracing::debug!("Streaming and constructing table");
        // get header
        let mut column = Vec::new();
        let mut rows = Vec::new();
        loop {
            let Some(RetrieveQueryResponse { kind }) = query_response_body
                .message()
                .await
                .map_err(retrieve_query_internal_error)?
            else {
                break;
            };
            let kind = kind.ok_or_else(invalid_response_type_error)?;

            match kind {
                Kind::Header(header) => column = header.cells,
                Kind::Row(data_row) => {
                    rows.push(data_row.cells.into_iter().map(|c| c.value).collect())
                }
            }
        }

        Ok(Table { column, rows })
    }

    async fn same<'ctx>(&self, ctx: &Context<'ctx>) -> Result<bool> {
        ctx.require_scope(Scope::ReadResource)?;

        let pool = ctx.data::<db::Pool>()?;
        let mut dbrunner = ctx.data::<rpc::DbRunnerClient>()?.clone();

        tracing::debug!(query_id = self.user_query_id, "Checking answer");
        let answer = db::get_question_answer(pool, 1)
            .await
            .map_err(error::gqlize)?;

        tracing::debug!(
            initial_sql = self.initial_sql,
            answer,
            "Running answer query"
        );
        let result = dbrunner
            .run_query(RunQueryRequest {
                schema: self.initial_sql.clone(),
                query: answer,
            })
            .await
            .map_err(|e| error::Error {
                code: error::ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("Unable to check answer."),
                error: Some(Box::new(e)),
            })?;
        let answer_sql_id = match result.into_inner().response_type {
            Some(ResponseType::Id(user_query_id)) => user_query_id,
            Some(ResponseType::Error(error)) => {
                return Err(error::Error {
                    code: error::ErrorCode::InternalError,
                    title: EcoString::inline("Internal error"),
                    details: Cow::Borrowed("Unable to check answer."),
                    error: Some(Box::new(AnswerExecutionError { error })),
                }
                .into())
            }
            None => return Err(invalid_response_type_error()),
        };

        tracing::debug!(answer_sql_id, "Comparing results");
        let comparison_result = dbrunner
            .are_queries_output_same(AreQueriesOutputSameRequest {
                left_id: self.user_query_id.clone(),
                right_id: answer_sql_id.clone(),
            })
            .await
            .map_err(retrieve_query_internal_error)?;
        let same = comparison_result.into_inner().same;

        tracing::debug!(
            same,
            left_id = self.user_query_id,
            right_id = answer_sql_id,
            "Done comparsion"
        );
        Ok(same)
    }
}

fn retrieve_query_internal_error(e: tonic::Status) -> error::Error {
    error::Error {
        code: error::ErrorCode::InternalError,
        title: EcoString::inline("Internal error"),
        details: Cow::Borrowed("Unable to retrieve query results."),
        error: Some(Box::new(e)),
    }
}

fn invalid_response_type_error() -> async_graphql::Error {
    error::Error {
        code: error::ErrorCode::InternalError,
        title: EcoString::inline("Internal error"),
        details: Cow::Borrowed("Unknown response type."),
        error: None,
    }
    .into()
}

#[derive(Debug)]
struct AnswerExecutionError {
    error: String,
}

impl std::fmt::Display for AnswerExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Answer execution failed: {}", self.error)
    }
}

impl std::error::Error for AnswerExecutionError {}

#[derive(SimpleObject)]
pub struct Table {
    pub column: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
}

#[derive(SimpleObject)]
pub struct ExecuteFailedResult {
    pub error: String,
}
