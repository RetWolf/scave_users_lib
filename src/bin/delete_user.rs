extern crate scave_users_lib;
extern crate diesel;

use self::diesel::prelude::*;
use self::scave_users_lib::*;
use std::env::args;

fn main() {
  use scave_users_lib::schema::users::dsl::*;

  let target = args().nth(1).expect("Expected a user ID to delete")
    .parse::<i32>().expect("Invalid ID");
  let connection = establish_connection();

  let num_deleted = diesel::delete(users.filter(id.eq(target)))
    .execute(&connection)
    .expect("Error deleting user");

  println!("Deleted {} user with id {}", num_deleted, target);
}