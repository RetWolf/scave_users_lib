extern crate scave_users_lib;
extern crate diesel;

use self::scave_users_lib::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
  use schema::users::dsl::*;

  let connection = establish_connection();
  let results = users.filter(logins.eq(0))
    .limit(5)
    .load::<User>(&connection)
    .expect("Error loading users");

  println!("Displaying {} users", results.len());
  for user in results {
    println!("{}", user.id);
    println!("-------------------\n");
    println!("{}", user.email);
  }
}