use super::auth::AuthBuilder;
use super::error::{Error, ErrorCode};
use super::{Mutation, Query};
use async_graphql::{EmptySubscription, Pos, Response, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use ecow::EcoString;
use poem::handler;
use poem::http::HeaderMap;
use poem::web::Data;

#[handler]
pub async fn index(
    schema: Data<&Schema<Query, Mutation, EmptySubscription>>,
    auth_builder: Data<&AuthBuilder>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;

    if let Some(token) = extract_jwt(headers) {
        let auth = auth_builder.build(&token).await.map_err(|e| Error {
            code: ErrorCode::InvalidJwtToken,
            title: EcoString::inline("Invalid token"),
            details: e.to_string().into(),
            error: Some(Box::new(e)),
        });

        match auth {
            Ok(auth) => {
                req = req.data(auth);
            }
            Err(e) => {
                // fixme: a bit ugly 🤔
                return GraphQLResponse(Response::from_errors(vec![e
                    .to_gql_error()
                    .into_server_error(Pos::default())]));
            }
        }
    }

    schema.execute(req).await.into()
}

fn extract_jwt(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            header
                .strip_prefix("Bearer ")
                .map(|token| token.to_string())
        })
}
