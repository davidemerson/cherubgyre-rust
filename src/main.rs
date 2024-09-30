use actix_web::{web, App, HttpServer};
use handlers::{register_user, create_invite};

mod handlers;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register_user))
            .route("/invite", web::post().to(create_invite)) // New endpoint for creating invites
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
