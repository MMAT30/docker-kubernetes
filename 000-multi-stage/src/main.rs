#[macro_use] extern crate rocket;
use rocket::serde::json::{json, Value};
use rocket::response::status::NoContent;

#[get("/", format = "json")]
fn index() -> Value {
    json!({
        "status": "ok",
        "message": "Hello, world!"
    })
}

#[post("/name/<id>", format = "json")]
fn get_name(id: i32) -> Value {
    json!({
        "status": "ok",
        "message": format!("Hello, {}!", id)
    })
}

#[get("/name/<id>", format = "json")]
fn name(id: i32) -> Value {
    json!({
        "status": "ok",
        "message": format!("Hello, {}!", id)
    })
}

#[delete("/name/<id>", format = "json")]
fn delete_name(id: i32) -> Value {
    json!({
        "status": "ok",
        "message": format!("{} deleted!", id)
    })
}

#[get("/notfound", format = "json")]
fn not_found() -> NoContent {
    return NoContent;
}

#[catch(404)]
fn catch_404() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", routes![
        index,
        get_name,
        name, 
        delete_name,
        not_found
    ])
    .register("/", catchers![catch_404])
    .launch()
    .await;
}