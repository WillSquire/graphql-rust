use rocket::response::content::Html;
use rocket::State;
use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
use crate::graphql::schema::{Context, Schema};

#[get("/")]
pub fn graphiql() -> Html<String> {
  graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
  context: State<Context>,
  request: GraphQLRequest,
  schema: State<Schema>,
) -> GraphQLResponse {
  request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
  context: State<Context>,
  request: GraphQLRequest,
  schema: State<Schema>,
) -> GraphQLResponse {
  request.execute(&schema, &context)
}
