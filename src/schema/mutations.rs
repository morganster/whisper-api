use async_graphql::MergedObject;
use self::{users::UserMutation, twisters::TwisterMutation};

pub mod twisters;
pub mod users;
#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation, TwisterMutation);
