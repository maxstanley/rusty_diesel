use actix_web::{App, HttpServer};
mod user;
mod version;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API Server");

    HttpServer::new(move || {
        App::new()
            .service(version::version_routes())
            .service(user::user_routes())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
