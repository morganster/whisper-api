use crate::{
    schema::models::posts::{CreatePost, CreatedResponse},
    utils::security::{get_token_data, Token},
    ApiContext, ApiError,
};
use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    async fn post_whisp<'a>(
        &self,
        ctx: &'a Context<'_>,
        post: CreatePost,
    ) -> Result<CreatedResponse, ApiError> {
        let token = ctx.data_opt::<Token>().unwrap();
        //TODO: move to a public function
        let token_data = get_token_data(token)?;
        let claims = token_data.claims;
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;

        let inserted_post = sqlx::query!(
            r"INSERT INTO posts (content, user_id)
             VALUES (?, ?)",
            post.content,
            claims.sub,
        )
        .execute(conn)
        .await;
        match inserted_post {
            Ok(post) => Ok(CreatedResponse {
                id: post.last_insert_id(),
            }),
            Err(_e) => Err(ApiError::ExpiredToken),
        }
    }
}
