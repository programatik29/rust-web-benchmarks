use rocket::{get, launch, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
