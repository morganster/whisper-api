use async_graphql::MergedObject;
use self::{users::UserMutation, posts::PostMutation};

pub mod posts;
pub mod users;
#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation, PostMutation);
