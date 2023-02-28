use async_graphql::MergedObject;

use self::{users::UserQuery, posts::PostQuery};

pub mod users;
pub mod posts;


#[derive(MergedObject, Default)]
pub struct QueryRoot( UserQuery,PostQuery);
