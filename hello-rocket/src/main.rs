#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod other {
  #[get("/world")]
  pub fn world() -> &'static str {
     "other Hello world"
  }
}

#[get("/")]
fn index() -> &'static str {
    "Hello,world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, other::world]).launch();
}
