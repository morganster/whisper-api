pub mod queries;
pub mod mutations;
pub mod models;

use queries::QueryRoot;
use mutations::MutationRoot;
use async_graphql::{
   EmptySubscription, Schema,
};
pub type WhisperSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
