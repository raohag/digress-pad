
#[openapi]
#[get("/ping")]
pub fn ping() -> String {
    format!("Pong!")
}

#[openapi]
#[get("/version")]
pub fn version() -> String {
    format!("0.1.0")
}
