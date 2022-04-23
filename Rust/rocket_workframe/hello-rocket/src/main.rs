// section 01

#[macro_use] extern crate rocket;

#[get("/")]
fn no_hello() -> &'static str {
    "No hello world!!!"
}

#[get("/h")]  // localhost/hello/h
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
    .mount("/hello", routes![world, no_hello])
    .launch()
    .await;
}


// section 02

// #[mecro_use] extern crate rocket;
// use rocket::tokio::time::{sleep, Duration};

// #[rocket::main]
// async fn main(){
//     rocket::build()
//     .mount("/")
// }

// #[get("/delay/<seconds>")]
// async fn delay(seconds: u64) -> String {
//     sleep(Duration::from_secs(seconds)).await;
//     format!("Waited for {} seconds", seconds)
// }