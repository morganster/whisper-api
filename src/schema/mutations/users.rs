use crate::schema::models::users::{LoginResponse, LoginUser, RegisterResponse, SimpleUser, User};
use crate::utils::security::{get_hashed_password, get_jwt_for_user, verify_password};
use crate::{ApiContext, ApiError};
use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        user: SimpleUser,
    ) -> Result<RegisterResponse, ApiError> {
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
        .await;
        match response {
            Ok(user) => Ok(RegisterResponse {
                id: user.last_insert_id(),
            }),
            Err(_e) => Err(ApiError::ExpiredToken),
        }
    }

    async fn login(&self, ctx: &Context<'_>, user: LoginUser) -> Result<LoginResponse, ApiError> {
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let mut token = "".to_string();
        let mut error = "".to_string();
        let hashed_password = get_hashed_password(&user.password);
        let q_user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = ?",
            user.username
        )
        .fetch_one(conn)
        .await;

        match q_user {
            Ok(user) => {
                if verify_password(&user.password, &hashed_password) {
                    token = get_jwt_for_user(&user);
                } else {
                    error = "user not found or yes".to_string();
                }
                Ok(LoginResponse {
                    token: Some(token),
                    error: Some(error),
                })
            }
            Err(_e) => Err(ApiError::WrongCredentials),
        }
    }
}
