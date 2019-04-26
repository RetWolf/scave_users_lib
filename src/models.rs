#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub user_password: String,
  pub logins: i32,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub email: &'a str,
  pub user_password: &'a str,
}
