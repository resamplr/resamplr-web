#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
  NamedFile::open("public/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("public/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
  rocket::ignite().mount("/", routes![index, files])
}

fn main() {
  rocket().launch();
}