use std::net::SocketAddr;

use async_graphql::{extensions::Tracing, http::GraphiQLSource, EmptySubscription, Schema};
use backend::{
    gql::{self, auth::AuthBuilder},
    rpc,
};
use mimalloc_rust::GlobalMiMalloc;
use poem::{listener::TcpListener, web::Html, *};

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[handler]
async fn health() -> impl IntoResponse {
    "OK"
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().expect("invalid port")));

    let logto_domain = std::env::var("LOGTO_DOMAIN")
        .expect("LOGTO_DOMAIN must be set")
        .into();
    let logto_resource_indicator = std::env::var("LOGTO_RESOURCE_INDICATOR")
        .expect("LOGTO_RESOURCE_INDICATOR must be set")
        .into();
    let auth_builder = AuthBuilder {
        logto_domain,
        resource_indicator: logto_resource_indicator,
    };

    let schema = Schema::build(
        gql::Query::default(),
        gql::Mutation::default(),
        EmptySubscription,
    )
    .data(backend::db::pool().await?)
    .data(rpc::dbrunner_client().await?)
    .extension(Tracing)
    .finish();

    let app = Route::new()
        .at("/", get(graphiql).post(gql::poem::index))
        .at("/health", get(health))
        .data(auth_builder)
        .data(schema);

    tracing::info!(
        "GraphiQL: http://127.0.0.1:{port}. Listened on {addr}",
        port = port,
        addr = addr
    );
    Server::new(TcpListener::bind(addr)).run(app).await?;
    Ok(())
}
