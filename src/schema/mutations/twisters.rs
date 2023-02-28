use std::sync::Arc;

use async_graphql::{Context, Object};

use crate::{
    schema::models::{
        twisters::{CreateTwist, CreatedResponse},
    },
    utils::security::{ Token, get_token_data},
    ApiContext, ApiError,
};

#[derive(Default)]
pub struct TwisterMutation;

#[Object]
impl TwisterMutation {
    async fn post_twister<'a>(
        &self,
        ctx: &'a Context<'_>,
        twist: CreateTwist,
    ) -> Result<CreatedResponse, ApiError> {
        let token = ctx.data_opt::<Token>().unwrap();
        //TODO: move to a public function
        let token_data = get_token_data(token)?;
        let claims = token_data.claims;
        let client = ctx.data_unchecked::<Arc<ApiContext>>();
        let conn = &client.db;

        let inserted_twist = sqlx::query!(
            r"INSERT INTO twisters (content, user_id)
             VALUES (?, ?)",
            twist.content,
            claims.sub,
        )
        .execute(conn)
        .await;
        match inserted_twist {
            Ok(twist) => Ok(CreatedResponse { id: twist.last_insert_id() }),
            Err(e) => Err(ApiError::ExpiredToken),      
        }
    }
}
