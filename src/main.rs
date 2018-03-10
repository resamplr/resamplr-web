#![feature(plugin)]
#![plugin(rocket_codegen)]

/// ORM and config
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] 
extern crate serde_derive;

/// Connection pooling 
extern crate r2d2;
extern crate r2d2_diesel;

// Web server
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod db;
mod schema;
mod controllers;
mod models;

use rocket::response::NamedFile;
use rocket::Rocket;
use std::path::{Path, PathBuf};
use dotenv::dotenv;
use std::env;
use controllers::products;

fn rocket() -> Rocket {
    dotenv().ok();

    // We need to make sure our database_url is set in our `.env` file. This will point to
    // our Postgres database.  If none is supplied, the program will error.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initializes database pool with r2d2.
    let pool = db::init_pool(database_url);

    // Configure our server, and mount all routes.  We don't "launch" the server
    // here, but in our `main` procedure.
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![
            root, 
            files, 
            products::get,
            products::index,

            ])
}

/// Main program
#[get("/")]
fn root() -> std::io::Result<NamedFile> {
    NamedFile::open("public/index.html")
}

/// Any public file can be requested here.  That includes all JS and CSS files.
#[get("/public/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

/// Launch our webserver
fn main() {
    rocket().launch();
}
