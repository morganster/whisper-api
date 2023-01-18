use crate::schema::models::users::{User, SimpleUser, RegisterResponse, LoginUser, LoginResponse};
use crate::utils::security::{get_hashed_password, get_jwt_for_user, verify_password, Token};
use crate::ApiContext;
use async_graphql::{
    Context, Object,
};
use std::sync::Arc;

use super::MutationRoot;




#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, user: SimpleUser) -> RegisterResponse {
        let photo = user.photo.unwrap_or("".to_string());
        let password = get_hashed_password(&user.password);
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let response = sqlx::query!(
            r"INSERT INTO users (username, password, photo,email,full_name)
        VALUES (?, ?, ?, ?, ?)",
            user.username,
            password,
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

    async fn login(&self, ctx: &Context<'_>, user: LoginUser) -> LoginResponse {
        let token = ctx.data_unchecked::<Token>();

        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let mut token = "".to_string();
        let mut error = "".to_string();
        let hashed_password = get_hashed_password(&user.password);
        let q_user: User = sqlx::query_as!(
            User,
            r"SELECT * FROM users WHERE username = ?",
            user.username
        )
        .fetch_one(conn)
        .await
        .unwrap();// TODO: Error handling

        if verify_password(&user.password, &hashed_password) {
            token = get_jwt_for_user(&q_user);
        } else {
            error = "user not found or yes".to_string();
        }

        LoginResponse {
            token: Some(token),
            error: Some(error),
        }
    }
}
