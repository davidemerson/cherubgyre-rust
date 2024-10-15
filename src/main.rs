use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use aws_sdk_dynamodb::Client;
use db::{save_user, save_invite, get_dynamodb_client}; // Import your db functions
use handlers::{register_user, create_invite};
use follow_handlers::{follow_user, unfollow_user, get_followers, delete_follower};
use duress_handlers::{
	trigger_duress, cancel_duress, enable_test_mode, get_map_info, get_preferences,
	update_preferences,
};
use tracing::{info, error};
use dotenv::dotenv;

mod db;
mod duress_db;
mod duress_handlers;
mod follow_db;
mod follow_handlers;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	tracing_subscriber::fmt::init();

	// Initialize DynamoDB client
	let client = get_dynamodb_client().await;

	let func = service_fn(|event: LambdaEvent<Value>| {
		// Handle API Gateway requests here
		async move {
			let (event, _context) = event.into_parts();
			let response = handle_request(event, client.clone()).await;
			Ok::<_, Error>(response)
		}
	});

	lambda_runtime::run(func).await?;
	Ok(())
}

async fn handle_request(event: Value, client: Client) -> Value {
	// Parse the event and route it to the appropriate handler
	match event["path"].as_str() {
		Some("/register") => register_user(client, event).await,
		Some("/invite") => create_invite(client, event).await,
		Some(path) if path.starts_with("/users") => {
			// Handle other user-related routes like follow, unfollow, etc.
			// Similar logic for other handlers
			json!({ "message": "Handling user routes" })
		}
		_ => json!({
			"statusCode": 404,
			"body": "Not Found"
		}),
	}
}
