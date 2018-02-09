#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]              
fn root() -> &'static str {  
    "Hello, world!"
}

fn main() {
  // error[E0425]: cannot find value `static_rocket_route_info_for_world` in this scope
  rocket::ignite().mount("/", routes![root]).launch();
}