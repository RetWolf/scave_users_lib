extern crate scave_users_lib;
extern crate diesel;

use self::scave_users_lib::*;
use std::io::{stdin, Read};

fn main() {
  let connection = establish_connection();

  println!("What is your email?");
  let mut email = String::new();
  stdin().read_line(&mut email).unwrap();
  let email = &email[..(email.len() - 1)];
  println!("\nOk! What would you like your password to be?");
  let mut password = String::new();
  stdin().read_line(&mut password).unwrap();
  let password = &password[..(password.len() - 1)];

  let user = create_user(&connection, email, password);
  println!("\nCreated user {} with id {}", email, user.id);
}