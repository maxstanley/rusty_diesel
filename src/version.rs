use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Version {
    version: String,
}

pub fn version_routes() -> Scope {
    web::scope("/version/").service(get)
}

#[get("/")]
pub async fn get(_request: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(Version {
        version: "0.0.1".to_string(),
    })
}
