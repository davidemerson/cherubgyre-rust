use actix_web::{web, App, HttpServer};
use aws_sdk_dynamodb::Client;
use db::{save_user, save_invite, get_dynamodb_client}; // Import your db functions
use handlers::{register_user, create_invite}; // Assuming you have handler implementations
use follow_handlers::{follow_user, unfollow_user, get_followers, delete_follower};
use duress_handlers::{trigger_duress, cancel_duress, enable_test_mode, get_map_info, get_preferences, update_preferences};
use tracing::{info, error};
use tracing_subscriber;
use dotenv::dotenv;

mod handlers;
mod db;
mod follow_handlers; // Add this line
mod duress_db;
mod duress_handlers;
mod follow_db;       // Add this line

#[actix_web::main]
async fn main() -> std::io::Result<()> { 
    // Initialize DynamoDB client
    let client = get_dynamodb_client().await;
    dotenv().ok();

    tracing_subscriber::fmt::init();

    info!("Starting server...");

    HttpServer::new(move || {
        // Clone the client to make it available in the app state
        let client_clone = client.clone();

        App::new()
            .app_data(web::Data::new(client_clone)) // Store DynamoDB client in app state
            .route("/health", web::get().to(|| async { "System is Live" }))
            .route("/register", web::post().to(register_user)) // Register endpoint
            .route("/invite", web::post().to(create_invite)) // Create invite endpoint
            .service(
                web::scope("/users")
                .route("/{user_id}/follow", web::post().to(follow_user))
                .route("/{user_id}/unfollow", web::post().to(unfollow_user))
                .route("/{user_id}/followers", web::get().to(get_followers))
                .route("/{user_id}/followers/{follower_id}", web::delete().to(delete_follower))
                .route("/{user_id}/duress", web::post().to(trigger_duress))
                .route("/{user_id}/duress/cancel", web::post().to(cancel_duress))
                .route("/{user_id}/test-mode", web::post().to(enable_test_mode))
                .route("/{user_id}/map", web::get().to(get_map_info))
                .route("/{user_id}/preferences", web::get().to(get_preferences))
                .route("/{user_id}/preferences", web::patch().to(update_preferences))

           )
            // Add more routes as needed
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
