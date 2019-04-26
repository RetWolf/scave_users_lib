extern crate scave_users_lib;
extern crate diesel;

use self::diesel::prelude::*;
use self::scave_users_lib::*;
use self::models::User;
use std::env::args;

fn main() {
  use scave_users_lib::schema::users::dsl::{users, user_password};

  let id = args().nth(1).expect("Need user id to update user")
    .parse::<i32>().expect("Invalid ID");
  let new_password = args().nth(2).expect("Need a new password to update password")
    .parse::<String>().expect("Invalid Password");
  let connection = establish_connection();

  let user = diesel::update(users.find(id))
    .set(user_password.eq(&new_password))
    .get_result::<User>(&connection)
    .expect(&format!("Unable to find user {}", id));

  println!("Updated user {} with new password: {}", user.id, user.user_password);
}