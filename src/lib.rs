pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set.");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewUser, User};

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, user_password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        email: email,
        user_password: user_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
