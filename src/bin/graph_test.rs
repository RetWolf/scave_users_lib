#[macro_use]
extern crate juniper;
extern crate scave_users_lib;

use juniper::{FieldResult, Variables, EmptyMutation};
use scave_users_lib::models::User;

struct Query;

graphql_object!(Query: Ctx |&self| {
  field getUser(&executor) -> FieldResult<User> {
    let user = executor.context().0.clone();
    Ok(user)
  }
});

struct Ctx(User);

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>>;

fn main() {
  let user = User {
    id: 2,
    email: String::from("mattconway55@gmail.com"),
    user_password: String::from("testing"),
    logins: 0,
  };
  let ctx = Ctx(user);

  let (res, _errors) = juniper::execute(
    "query {
      getUser {
        id
        email
      }
    }", 
    None, 
    &Schema::new(Query, EmptyMutation::new()), 
    &Variables::new(), 
    &ctx
  ).unwrap();

  println!("{:#?}", res.as_object_value().unwrap().get_field_value("getUser"));
}
