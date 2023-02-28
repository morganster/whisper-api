mod schema;
mod utils;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use schema::WhisperSchema;
use sqlx::MySqlPool;
use thiserror::Error;
use utils::security::Token;
use crate::schema::{mutations::MutationRoot, queries::QueryRoot};
use axum::{
    async_trait,
    extract::{Extension, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    RequestPartsExt, Router, TypedHeader,
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
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
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

async fn graphql_handler(
    schema: Extension<WhisperSchema>,
    token: Token,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    req = req.data(token);

    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::MissingHeader)?;
        Ok(utils::security::Token(bearer.token().to_string()))
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("missing credentials")]
    MissingCredentials,
    #[error("an error occured during token creation")]
    TokenCreation,
    #[error("invalid token")]
    InvalidToken,
    #[error("missing header")]
    MissingHeader,
    #[error("Expired Token")]
    ExpiredToken,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::ExpiredToken => StatusCode::UNAUTHORIZED,
            ApiError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidToken => StatusCode::BAD_REQUEST,
            ApiError::MissingHeader => StatusCode::BAD_REQUEST,
            ApiError::WrongCredentials => StatusCode::BAD_REQUEST,
            ApiError::MissingCredentials => StatusCode::BAD_REQUEST,
        };

        (status, self.to_string()).into_response()
    }
}
