#[macro_use]
extern crate juniper;
extern crate diesel;
extern crate scave_users_lib;

use diesel::prelude::*;
use diesel::r2d2::*;
use dotenv::dotenv;
use juniper::{EmptyMutation, FieldResult, Variables};
use scave_users_lib::models::User;
use scave_users_lib::schema;
use std::env;

struct Context {
  pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl juniper::Context for Context {}

struct Query;
graphql_object!(Query: Context |&self| {
  field apiVersion() -> &str {
    "1.0"
  }

  field getUser(&executor, user_id: i32) -> FieldResult<User> {
    use schema::users::dsl::*;
    let context = executor.context();
    let connection = context.pool.get()?;
    let user = users.filter(id.eq(user_id))
      .first::<User>(&connection)
      .expect("Error finding user");

    Ok(user)
  }
});

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

fn main() {
  dotenv().ok();

  let database_url =
    env::var("DATABASE_URL").expect("Database URL environment variable must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create database pool");

  let context = Context { pool: pool };

  let (res, _errors) = juniper::execute(
    "query {
      getUser(userId: 3) {
        id
        email
        logins
      }
    }",
    None,
    &Schema::new(Query, EmptyMutation::new()),
    &Variables::new(),
    &context,
  )
  .unwrap();

  println!(
    "{:#?}",
    res.as_object_value().unwrap().get_field_value("getUser")
  );
}
