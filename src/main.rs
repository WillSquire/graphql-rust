#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;

mod graphql;
mod models;
mod routes;

fn main() {
  rocket::ignite()
    .manage(graphql::schema::Context{})
    .manage(graphql::schema::Schema::new(
      graphql::schema::QueryRoot,
      graphql::schema::MutationRoot,
    ))
    .mount("/", routes![
      routes::graphql::graphiql,
      routes::graphql::get_graphql_handler,
      routes::graphql::post_graphql_handler
    ])
    .launch();
}
