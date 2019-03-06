#[derive(GraphQLObject)]
pub struct User {
  pub created: i32,
  pub name: String,
}

#[derive(GraphQLInputObject)]
pub struct UserCreate {
  pub created: i32,
  pub name: String,
  pub email: String,
}
