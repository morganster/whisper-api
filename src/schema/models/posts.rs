use async_graphql::{Object, SimpleObject, InputObject};
use sqlx::types::time;



pub struct Post {
  pub id: u64,
  pub content: Option<String>,
  pub created_at: time::PrimitiveDateTime,
  pub updated_at: time::PrimitiveDateTime,
  pub user_id: Option<u32>,
  pub reply_to: Option<u64>,
}

#[Object]
impl Post {
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

#[derive(Clone, SimpleObject, Default)]
pub struct CreatedResponse {
    pub id: u64,
}


#[derive(InputObject)]
pub struct CreatePost {
    pub content: String,
}
