#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_okapi;

mod rest;

#[launch]
fn rocket() -> _ {
    println!("Starting Engines!");
    rocket::build().mount("/", routes![rest::health::ping, rest::health::version])
}
