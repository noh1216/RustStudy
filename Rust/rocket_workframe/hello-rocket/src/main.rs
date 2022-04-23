#[macro_use] extern crate rocket;

#[get("/")]
fn no_hello() -> &'static str {
    "No hello world!!!"
}

#[get("/h")]
fn world() -> &'static str {
    "Hello, world!"
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/hello", routes![world, no_hello])
// }

#[rocket::main]
async fn main() {
    rocket::build()
    .mount("/", routes![world, no_hello])
    .launch()
    .await;
}