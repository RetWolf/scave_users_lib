#[macro_use]
extern crate juniper;
extern crate diesel;
extern crate scave_users_lib;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::{EmptyMutation, FieldResult};
use scave_users_lib::models::User;
use scave_users_lib::schema;
use std::sync::Arc;
use std::{env, vec::Vec};

struct Context {
  pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

struct QueryRoot;
graphql_object!(QueryRoot: Context |&self| {
  field getUsers(&executor) -> FieldResult<Vec<User>> {
    use schema::users::dsl::*;
    let context = executor.context();
    let connection = context.pool.get().unwrap();
    let results = users.filter(logins.eq(0)).load::<User>(&connection).expect("Error loading users");
    Ok(results)
  }
});

type Schema = juniper::RootNode<'static, QueryRoot, EmptyMutation<Context>>;

fn graphiql() -> HttpResponse {
  let gql_editor = graphiql_source("http://127.0.0.1:8080/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(gql_editor)
}

fn graphql(
  data: web::Data<ApiData>,
  request: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || {
    let res = request.execute(&data.schema, &data.context); // Error here - need to pass in GraphQL context properly
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .map_err(Error::from)
  .and_then(|user| {
    Ok(
      HttpResponse::Ok()
        .content_type("applicaton/json")
        .body(user),
    )
  })
}

struct ApiData {
  context: Context,
  schema: Arc<Schema>,
}

fn main() -> std::io::Result<()> {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let database_url =
    env::var("DATABASE_URL").expect("Database URL environment variable must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create database pool");

  let schema = Schema::new(QueryRoot, EmptyMutation::new());
  let arc_schema = Arc::new(schema);

  HttpServer::new(move || {
    App::new()
      .data(ApiData {
        context: Context { pool: pool.clone() },
        schema: arc_schema.clone(),
      })
      .wrap(middleware::Logger::default())
      .service(web::resource("/graphql").route(web::post().to_async(graphql)))
      .service(web::resource("/graphiql").route(web::get().to(graphiql)))
  })
  .bind("127.0.0.1:8080")?
  .run()
}
