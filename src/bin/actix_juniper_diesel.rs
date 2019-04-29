#[macro_use]
extern crate juniper;
extern crate scave_users_lib;

use scave_users_lib::models::User;
use std::sync::Arc;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::{FieldResult, EmptyMutation};

struct QueryRoot;
graphql_object!(QueryRoot: Ctx |&self| {
  field getUser(&executor) -> FieldResult<User> {
    let user = executor.context().0.clone();
    Ok(user)
  }
});

struct Ctx(User);

type Schema = juniper::RootNode<'static, QueryRoot, EmptyMutation<Ctx>>;

fn graphiql() -> HttpResponse {
  let gqlEditor = graphiql_source("http://127.0.0.1:8080/graphql");
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(gqlEditor)
}

fn graphql(
  root: web::Data<Arc<Schema>>,
  data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || {
    let res = data.execute(&root, &()); // Error here - need to pass in GraphQL context properly
    Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
  })
  .map_err(Error::from)
  .and_then(|user| {
    Ok(HttpResponse::Ok()
      .content_type("applicaton/json")
      .body(user)
    )
  })
}

fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let schema = std::sync::Arc::new(Schema::new(QueryRoot, EmptyMutation::new()));

  HttpServer::new(move || {
    App::new()
      .data(schema.clone())
      .wrap(middleware::Logger::default())
      .service(web::resource("/graphql").route(web::post().to_async(graphql)))
      .service(web::resource("/graphiql").route(web::get().to(graphiql)))
  })
  .bind("127.0.0.1:8080")?
  .run()
}