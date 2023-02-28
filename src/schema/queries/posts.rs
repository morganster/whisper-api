use crate::{schema::models::posts::Post, ApiContext};
use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn posts(&self, ctx: &Context<'_>) -> Vec<Post> {
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let posts: Vec<Post> = sqlx::query_as!(Post, r"select * from posts")
            .fetch_all(conn)
            .await
            .unwrap_or(Vec::new());
        posts
    }
}
