use async_graphql::{Context, EmptySubscription, InputObject, Object, Schema, SimpleObject};
use sqlx::types::time;
use std::sync::Arc;

use crate::ApiContext;
pub type UserSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone)]
pub struct User {
    id: u32,
    username: String,
    password: String,
    photo: Option<String>,
    email: String,
    full_name: Option<String>,
    created_at: time::PrimitiveDateTime,
    updated_at: time::PrimitiveDateTime,
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

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let users: Vec<User> = sqlx::query_as!(User, r"select * from users")
            .fetch_all(conn)
            .await
            .unwrap_or(Vec::new());
        users
    }
}

#[derive(InputObject)]
pub struct SimpleUser {
    username: String,
    password: String,
    photo: Option<String>,
    email: String,
    full_name: Option<String>,
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

#[derive(Clone, SimpleObject)]
pub struct RegisterResponse {
    id: u64,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, user: SimpleUser) -> RegisterResponse {
        let photo = user.photo.unwrap_or("".to_string());
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let response = sqlx::query!(
            r"INSERT INTO users (username, password, photo,email,full_name)
        VALUES (?, ?, ?, ?, ?)",
            user.username,
            user.password,
            photo,
            user.email,
            user.full_name.unwrap_or("".to_string())
        )
        .execute(conn)
        .await
        .unwrap();
        let id = response.last_insert_id();
        RegisterResponse { id: id }
    }
}
