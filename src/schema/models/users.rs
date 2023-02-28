use async_graphql::{InputObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::types::time;

#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub photo: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
    pub created_at: time::PrimitiveDateTime,
    pub updated_at: time::PrimitiveDateTime,
}

#[Object]
impl User {
    async fn id(&self) -> &u32 {
        &self.id
    }
    async fn username(&self) -> &str {
        &self.username
    }
    async fn password(&self) -> &str {
        &self.password
    }
    async fn photo(&self) -> &Option<String> {
        &self.photo
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn full_name(&self) -> &Option<String> {
        &self.full_name
    }
    async fn created_at(&self) -> std::string::String {
        self.created_at.to_owned().to_string()
    }
    async fn updated_at(&self) -> std::string::String {
        self.updated_at.to_owned().to_string()
    }
}

#[derive(InputObject)]
pub struct SimpleUser {
    pub username: String,
    pub password: String,
    pub photo: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
}

#[derive(InputObject)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[Object]
impl SimpleUser {
    async fn username(&self) -> &str {
        &self.username
    }
    async fn password(&self) -> &str {
        &self.password
    }
    async fn photo(&self) -> &Option<String> {
        &self.photo
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn full_name(&self) -> &Option<String> {
        &self.full_name
    }
}

#[Object]
impl LoginUser {
    async fn username(&self) -> &str {
        &self.username
    }
    async fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Clone, SimpleObject)]
pub struct RegisterResponse {
    pub id: u64,
}

#[derive(Clone)]
pub struct LoginResponse {
    pub token: Option<String>,
    pub error: Option<String>,
}

#[Object]
impl LoginResponse {
    async fn token(&self) -> &Option<String> {
        &self.token
    }
    async fn error(&self) -> &Option<String> {
        &self.error
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Claims {
    pub sub: u32,
    pub exp: usize,
}
