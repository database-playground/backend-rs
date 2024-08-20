use std::borrow::Cow;

use crate::{
    db,
    gql::error,
    rpc::{
        self,
        dbrunner::{
            retrieve_query_response::Kind, run_query_response::ResponseType, RetrieveQueryRequest,
            RetrieveQueryResponse, RunQueryRequest,
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

        let query_response = dbrunner
            .retrieve_query(RetrieveQueryRequest {
                id: self.user_query_id.clone(),
            })
            .await
            .map_err(retrieve_query_internal_error)?;
        let mut query_response_body = query_response.into_inner();

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

#[derive(SimpleObject)]
pub struct Table {
    pub column: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
}

#[derive(SimpleObject)]
pub struct ExecuteFailedResult {
    pub error: String,
}
