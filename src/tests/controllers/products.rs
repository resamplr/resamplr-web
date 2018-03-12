#[cfg(test)]
use rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn verify_run() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
