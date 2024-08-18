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

    // create the schema
    let schema = Schema::build(
        gql::Query(gql::schema::SchemaQuery),
        EmptyMutation,
        EmptySubscription,
    )
    .data(backend::db::pool().await?)
    .extension(Tracing)
    .finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    tracing::info!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
