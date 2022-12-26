use axum::{
    response::Html,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;



#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(hello_world));

        println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}