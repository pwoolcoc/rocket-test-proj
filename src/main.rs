#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{JSON, Value};

type ID = usize;

#[derive(Serialize, Deserialize)]
struct Foo {
    id: Option<ID>,
    name: String,
}

#[get("/foo", format = "application/json")]
fn foo() -> Option<JSON<Foo>> {
    Some(JSON(Foo {
        id: Some(0),
        name: "".to_string(),
    }))
}

fn main() {
    rocket::ignite().mount("/", routes![foo]).launch();
}
