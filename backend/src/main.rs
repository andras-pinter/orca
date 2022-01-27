#[macro_use]
extern crate rocket;
use common::IName;
use rocket::serde::json::Json;

#[post("/name", data = "<name>")]
fn name(name: Json<IName>) -> Json<IName> {
    Json(IName {
        name: name.name.to_uppercase(),
    })
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/api", routes![name])
        .launch()
        .await
        .unwrap();
}
