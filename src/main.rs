use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde_json::{json, Value};
use simple_logger::SimpleLogger;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let path = event.payload["requestContext"]["path"].as_str().unwrap_or_default();
    let method = event.payload["httpMethod"].as_str().unwrap_or_default();

    match (method, path) {
        ("POST", "/users/invite") => handlers::generate_invite_code(event).await,
        ("POST", "/users/register") => handlers::register_user(event).await,
        ("GET", path) if path.starts_with("/users/") && path.ends_with("/followers") => {
            handlers::get_followers(event).await
        }
        ("POST", path) if path.ends_with("/duress") => handlers::duress_alert(event).await,
        ("POST", path) if path.ends_with("/duress/cancel") => handlers::cancel_duress(event).await,
        ("POST", path) if path.ends_with("/test-mode") => handlers::enable_test_mode(event).await,
        ("GET", path) if path.ends_with("/map") => handlers::get_map_data(event).await,
        _ => Ok(json!({ "message": "Route not found" })),
    }
}
