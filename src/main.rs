mod schema;
mod utils;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use jsonwebtoken::{ decode, DecodingKey, Validation};
use schema::{models::users::Claims, TwisterSchema};
use sqlx::MySqlPool;
use thiserror::Error;
use utils::security::{ get_secret, Token};

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

// async fn get_token_from_headers(parts: &Parts) -> Claims {
//     let TypedHeader(Authorization(bearer)) = parts
//             .extract::<TypedHeader<Authorization<Bearer>>>()
//             .await
//             .map_err(|_| AuthError::InvalidToken);
//     let claims = get_claims_from_token(bearer.token().to_string());

//     Ok(claims)
//     // headers
//     //     .get("Authorization")
//     //     .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
// }

async fn graphql_handler(
    schema: Extension<TwisterSchema>,
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
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingHeader)?;
        Ok(utils::security::Token(bearer.token().to_string()))
        // Decode the user data
        // let token_data = decode::<Claims>(
        //     &bearer.token(),
        //     &DecodingKey::from_secret(&get_secret()),
        //     &Validation::default(),
        // ).map_err(|_| AuthError::InvalidToken)?;
        // Ok(token_data.claims)
    }
}


#[derive(Debug, Error)]
pub enum AuthError {
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
    ExpiredToken

}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::ExpiredToken => StatusCode::UNAUTHORIZED,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken => StatusCode::BAD_REQUEST,
            AuthError::MissingHeader => StatusCode::BAD_REQUEST,
            AuthError::WrongCredentials => StatusCode::BAD_REQUEST,
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,

        };

        (status, self.to_string()).into_response()
    }
}