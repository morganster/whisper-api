use std::sync::Arc;

use async_graphql::{Object, Context};

use crate::{schema::models::twisters::Twist, ApiContext};


#[derive(Default)]
pub struct  TwisterQuery;

#[Object]
impl TwisterQuery {
    async fn twisters(&self, ctx: &Context<'_>) -> Vec<Twist> {
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;
        let twisters: Vec<Twist> = sqlx::query_as!(Twist, r"select * from twisters")
            .fetch_all(conn)
            .await
            .unwrap_or(Vec::new());
        twisters
    }
}