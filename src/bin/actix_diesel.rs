extern crate diesel;
extern crate scave_users_lib;

use self::scave_users_lib::*;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use futures::Future;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn create_user_query(
  user_email: String,
  pool: web::Data<Pool>,
) -> Result<models::User, diesel::result::Error> {
  use scave_users_lib::schema::users::dsl::*;

  let new_user = models::NewUser {
    email: user_email.clone(),
    user_password: String::from("testactix"),
  };

  let conn: &PgConnection = &pool.get().unwrap();

  diesel::insert_into(users)
    .values(&new_user)
    .execute(conn)
    .unwrap();

  let mut items = users
    .filter(email.eq(&user_email))
    .load::<models::User>(conn)?;

  Ok(items.pop().unwrap())
}

fn add_user(
  email: web::Path<String>,
  pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
  web::block(move || create_user_query(email.into_inner(), pool)).then(|res| match res {
    Ok(user) => Ok(HttpResponse::Ok().json(user)),
    Err(_) => Ok(HttpResponse::InternalServerError().into()),
  })
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

  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .wrap(middleware::Logger::default())
      .service(web::resource("/add/{email}").route(web::get().to_async(add_user)))
  })
  .bind("127.0.0.1:8080")?
  .run()
}
