#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;

#[get("/")]              
fn root() -> std::io::Result<NamedFile> {  
    NamedFile::open("public/index.html")
}

fn main() {
  // error[E0425]: cannot find value `static_rocket_route_info_for_world` in this scope
  rocket::ignite().mount("/", routes![root]).launch();
}