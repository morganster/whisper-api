use std::sync::Arc;
use crate::{ApiContext, schema::models::users::User};
use async_graphql::{
  Context, Object, Result,
};

#[derive(Default)]
pub struct UserQuery;


#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let users: Vec<User> = sqlx::query_as!(User, r"select * from users")
            .fetch_all(conn)
            .await
            .unwrap_or(Vec::new());
        Ok(users)
    }
}