#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]              
fn root() -> std::io::Result<NamedFile> {  
    NamedFile::open("public/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn main() {
  // error[E0425]: cannot find value `static_rocket_route_info_for_world` in this scope
  rocket::ignite().mount("/", routes![root, files]).launch();
}

