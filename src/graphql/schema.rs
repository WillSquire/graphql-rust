use juniper::{Context as JuniperContext, FieldResult, RootNode};
use crate::models::user::{User, UserCreate};

pub struct Context {}

impl JuniperContext for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
  field user(&executor) -> FieldResult<User> {
    let user = User {
      created: 1551909311,
      name: "Test".to_string(),
    };
    Ok(user)
  }
});


pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
  field add_user(&executor, user_create: UserCreate) -> FieldResult<User> {
    let user = User {
      created: user_create.created,
      name: user_create.name,
    };
    Ok(user)
  }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
