#[macro_use] extern crate rocket;

#[get("/ping")]
fn ping() -> String {
    format!("Pong!")
}

#[launch]
fn rocket() -> _ {
    println!("Starting Engines!");
    rocket::build().mount("/", routes![ping])
}
