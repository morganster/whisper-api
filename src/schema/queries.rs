use async_graphql::MergedObject;

use self::{users::UserQuery, twisters::TwisterQuery};

pub mod users;
pub mod twisters;


#[derive(MergedObject, Default)]
pub struct QueryRoot( UserQuery,TwisterQuery);
