mod schema;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use sqlx::MySqlPool;

use crate::schema::users::{MutationRoot, QueryRoot, UserSchema};
use axum::{
    extract::Extension,
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

async fn graphql_handler(schema: Extension<UserSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
