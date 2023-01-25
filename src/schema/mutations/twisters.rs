use std::sync::Arc;

use async_graphql::{Context, Object};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    schema::models::{
        twisters::{CreateTwist, CreatedResponse},
        users::Claims,
    },
    utils::security::{get_secret, Token},
    ApiContext, AuthError,
};

#[derive(Default)]
pub struct TwisterMutation;

#[Object]
impl TwisterMutation {
    async fn post_twister<'a>(
        &self,
        ctx: &'a Context<'_>,
        twist: CreateTwist,
    ) -> Result<CreatedResponse, AuthError> {
        let token = ctx.data_opt::<Token>().unwrap();
        //TODO: move to a public function
        let token_data = decode::<Claims>(
            &token.0.to_string(),
            &DecodingKey::from_secret(&get_secret()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;
        let claims = token_data.claims;
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
        Ok::<CreatedResponse, AuthError>(CreatedResponse { id: id })
    }
}
