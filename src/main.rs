mod schema;
mod utils;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use schema::TwisterSchema;
use sqlx::MySqlPool;

use crate::{
    schema::{mutations::MutationRoot, queries::QueryRoot},
    utils::security::Token,
};
use axum::{
    extract::Extension,
    http::header::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;

#[derive(Clone)]
struct ApiContext {
    db: MySqlPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let client = MySqlPool::connect(&db_url)
        .await
        .expect("can't connect to db");
    let pool = Arc::new(ApiContext { db: client });
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn graphql_handler(
    schema: Extension<TwisterSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        req = req.data(token);
    }
    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
