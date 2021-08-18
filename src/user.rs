use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    name: String,
    email: String,
    roles: Vec<String>,
    password_bcrypt: String,
}

pub fn user_routes() -> Scope {
    web::scope("/user/").service(get_all).service(create)
}

#[post("/user")]
async fn create(user: web::Json<User>) -> impl Responder {
    println!("User {:?}", user);
    HttpResponse::Created().body("")
}

#[get("/user")]
async fn get_all() -> impl Responder {
    HttpResponse::Created().body("")
}
