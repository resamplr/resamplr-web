#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;

use rocket::response::NamedFile;
use rocket::Rocket;
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use std::env;

mod db;

fn rocket() -> Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initializes database pool with r2d2.
    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![root, files])
}

/// Main program
#[get("/")]
fn root() -> std::io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn main() {
    rocket().launch();
}
