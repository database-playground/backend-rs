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
        ctx.require_scope(Scope::Execution)?;

        let pool = ctx.data::<db::Pool>()?;
        let mut dbrunner = ctx.rpc_client()?;

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
                _ if e.code() == tonic::Code::InvalidArgument => Error::InvalidQuery(e),
                _ => Error::RetrieveFailed(e),
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
            None => Err(Error::InvalidResponseType.into()),
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
        let mut dbrunner = ctx.rpc_client()?;

        tracing::debug!(query_id = self.user_query_id, "Retrieving query results");
        let query_response = dbrunner
            .retrieve_query(RetrieveQueryRequest {
                id: self.user_query_id.clone(),
            })
            .await
            .map_err(Error::RetrieveFailed)?;
        let mut query_response_body = query_response.into_inner();

        tracing::debug!("Streaming and constructing table");
        // get header
        let mut column = Vec::new();
        let mut rows = Vec::new();
        loop {
            let Some(RetrieveQueryResponse { kind }) = query_response_body
                .message()
                .await
                .map_err(Error::RetrieveFailed)?
            else {
                break;
            };
            let kind = kind.ok_or(Error::InvalidResponseType)?;

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
        ctx.require_scope(Scope::ReadAnswer)?;

        let pool = ctx.data::<db::Pool>()?;
        let mut dbrunner = ctx.rpc_client()?;

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
            .map_err(Error::RetrieveFailed)?;
        let answer_sql_id = match result.into_inner().response_type {
            Some(ResponseType::Id(user_query_id)) => user_query_id,
            Some(ResponseType::Error(error)) => return Err(Error::AnswerInvalid { error }.into()),
            None => return Err(Error::InvalidResponseType.into()),
        };

        tracing::debug!(answer_sql_id, "Comparing results");
        let comparison_result = dbrunner
            .are_queries_output_same(AreQueriesOutputSameRequest {
                left_id: self.user_query_id.clone(),
                right_id: answer_sql_id.clone(),
            })
            .await
            .map_err(Error::RetrieveFailed)?;
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

#[derive(SimpleObject)]
pub struct Table {
    pub column: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
}

#[derive(SimpleObject)]
pub struct ExecuteFailedResult {
    pub error: String,
}

pub enum Error {
    /// The generic error of async-graphql.
    GenericError(async_graphql::Error),
    InvalidQuery(tonic::Status),
    RetrieveFailed(tonic::Status),
    InvalidResponseType,
    DbrunnerUnavailable,
    AnswerInvalid {
        error: String,
    },
}

impl From<Error> for async_graphql::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::GenericError(e) => e,
            Error::InvalidQuery(e) => error::Error {
                code: error::ErrorCode::InvalidQuery,
                title: EcoString::inline("Invalid query"),
                details: e.message().to_string().into(),
                error: Some(Box::new(e)),
            }
            .to_gql_error(),
            Error::RetrieveFailed(e) => error::Error {
                code: error::ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("Unable to retrieve results from dbrunner."),
                error: Some(Box::new(e)),
            }
            .to_gql_error(),
            Error::InvalidResponseType => error::Error {
                code: error::ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("Unknown response type."),
                error: None,
            }
            .to_gql_error(),
            Error::DbrunnerUnavailable => error::Error {
                code: error::ErrorCode::InternalError,
                title: EcoString::inline("Internal error"),
                details: Cow::Borrowed("Database runner is not available."),
                error: None,
            }
            .to_gql_error(),
            Error::AnswerInvalid { error } => error::Error {
                code: error::ErrorCode::InternalError,
                title: EcoString::inline("Invalid answer"),
                details: error.into(),
                error: None,
            }
            .to_gql_error(),
        }
    }
}

trait ContextExt {
    fn rpc_client(&self) -> Result<rpc::DbRunnerClient, Error>;
}

impl ContextExt for Context<'_> {
    fn rpc_client(&self) -> Result<rpc::DbRunnerClient, Error> {
        self.data::<Option<rpc::DbRunnerClient>>()
            .map_err(Error::GenericError)?
            .clone()
            .ok_or(Error::DbrunnerUnavailable)
    }
}
