#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

#[get("/")]
fn root() -> std::io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![root, files]).launch();
}
