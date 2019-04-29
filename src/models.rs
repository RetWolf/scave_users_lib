#[derive(Queryable, GraphQLObject, Clone, Debug, Serialize)]
#[graphql(description = "A Scave User from the database")]
pub struct User {
  pub id: i32,
  pub email: String,
  pub user_password: String,
  pub logins: i32,
}

use super::schema::users;

#[derive(Insertable, GraphQLInputObject)]
#[graphql(description = "Input to create a Scave user")]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub user_password: String,
}
