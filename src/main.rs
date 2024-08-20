use std::net::SocketAddr;

use async_graphql::{
    extensions::Tracing, http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::*;
use backend::gql;
use poem::{listener::TcpListener, web::Html, *};

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().expect("invalid port")));

    let schema = Schema::build(gql::Query::default(), EmptyMutation, EmptySubscription)
        .data(backend::db::pool().await?)
        .extension(Tracing)
        .finish();

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    tracing::info!(
        "GraphiQL: http://127.0.0.1:{port}. Listened on {addr}",
        port = port,
        addr = addr
    );
    Server::new(TcpListener::bind(addr)).run(app).await?;
    Ok(())
}
