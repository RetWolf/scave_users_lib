extern crate diesel;
extern crate scave_users_lib;

use self::scave_users_lib::*;
use std::env::args;

fn main() {
  let connection = establish_connection();

  let email = args()
    .nth(1)
    .expect("Need an email to register")
    .parse::<String>()
    .expect("Invalid Email");
  let password = args()
    .nth(2)
    .expect("Need a password to register")
    .parse::<String>()
    .expect("Invalid Password");

  let user = create_user(&connection, email, password);
  println!("\nCreated user {} with id {}", user.email, user.id);
}
