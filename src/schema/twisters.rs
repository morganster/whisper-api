use crate::utils::security::{get_claims_from_token, Token};
use crate::ApiContext;
use async_graphql::{Context, EmptySubscription, InputObject, Object, Result, Schema, SimpleObject};
use sqlx::types::time;
use std::sync::Arc;
pub type TwisterSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct Twist {
    id: u64,
    content: Option<String>,
    created_at: time::PrimitiveDateTime,
    updated_at: time::PrimitiveDateTime,
    user_id: Option<u32>,
    reply_to: Option<u64>,
}

#[Object]
impl Twist {
    async fn id(&self) -> &u64 {
        &self.id
    }
    async fn content(&self) -> &Option<std::string::String> {
        &self.content
    }
    async fn created_at(&self) -> std::string::String {
        self.created_at.to_owned().to_string()
    }
    async fn updated_at(&self) -> std::string::String {
        self.updated_at.to_owned().to_string()
    }
    async fn user_id(&self) -> &Option<u32> {
        &self.user_id
    }
    async fn reply_to(&self) -> &Option<u64> {
        &self.reply_to
    }
}

#[derive(Clone, SimpleObject)]
pub struct CreatedResponse {
    id: u64,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn twisters(&self, ctx: &Context<'_>) -> Result<Vec<Twist>> {
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let twisters: Vec<Twist> = sqlx::query_as!(Twist, r"select * from twisters")
            .fetch_all(conn)
            .await
            .unwrap_or(Vec::new());
        Ok(twisters)
    }
}


#[derive(InputObject)]
pub struct CreateTwist {
    content: String,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn post_twister(&self, ctx: &Context<'_>, twist: CreateTwist) -> Result<CreatedResponse> {
        let token = ctx.data_unchecked::<Token>();
        let claims = get_claims_from_token(token.0.to_string());
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;

        let response = sqlx::query!(
            r"INSERT INTO twisters (content, user_id)
VALUES (?, ?)",
            twist.content,
            claims.sub,
        )
        .execute(conn)
        .await
        .unwrap();
        let id = response.last_insert_id();
        Ok(CreatedResponse { id: id })
    }
}
