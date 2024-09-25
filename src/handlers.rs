use aws_sdk_dynamodb::Client;
use lambda_runtime::{Error, LambdaEvent};
use serde_json::{json, Value};
use log::info;

// Function to get followers of a user
pub async fn get_followers(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let client = get_dynamodb_client().await;

    let user_id = event.payload["pathParameters"]["user_id"].as_str().unwrap_or_default();

    // Query the DynamoDB table to fetch followers
    let result = client
        .query()
        .table_name("followers")
        .key_condition_expression("followee_id = :user_id")
        .expression_attribute_values(":user_id", user_id.into())
        .send()
        .await?;

    let followers: Vec<Value> = result
        .items
        .unwrap_or_default()
        .into_iter()
        .map(|item| json!({
            "follower_id": item.get("follower_id").unwrap().as_s().unwrap(),
            "username": item.get("username").unwrap().as_s().unwrap(),
            "avatar": item.get("avatar").unwrap().as_s().unwrap(),
        }))
        .collect();

    Ok(json!({ "followers": followers }))
}

// Function to handle duress alert
pub async fn duress_alert(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let client = get_dynamodb_client().await;

    let body = event.payload["body"].as_str().unwrap_or_default();
    let body_json: Value = serde_json::from_str(body)?;

    let user_id = event.payload["pathParameters"]["user_id"].as_str().unwrap_or_default();
    let duress_type = body_json["duress_type"].as_str().unwrap_or_default();
    let message = body_json["message"].as_str().unwrap_or_default();

    info!("Duress alert triggered by user: {}", user_id);

    // Store duress status in DynamoDB
    client
        .put_item()
        .table_name("duress_status")
        .item("user_id", user_id.into())
        .item("duress_type", duress_type.into())
        .item("message", message.into())
        .item("timestamp", body_json["timestamp"].clone().into())
        .send()
        .await?;

    // TODO: Notify followers or nearby users (implement notification logic)

    Ok(json!({ "status": "Duress alert sent" }))
}

// Function to cancel an active duress status
pub async fn cancel_duress(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let client = get_dynamodb_client().await;

    let body = event.payload["body"].as_str().unwrap_or_default();
    let body_json: Value = serde_json::from_str(body)?;

    let user_id = event.payload["pathParameters"]["user_id"].as_str().unwrap_or_default();
    let normal_pin = body_json["normal_pin"].as_str().unwrap_or_default();

    // Validate the normal pin by fetching the user
    let result = client
        .get_item()
        .table_name("users")
        .key("user_id", user_id.into())
        .send()
        .await?;

    let stored_pin = result.item.unwrap().get("normal_pin").unwrap().as_s().unwrap();
    if stored_pin != normal_pin {
        return Ok(json!({ "error": "Invalid pin code" }));
    }

    // Remove duress status
    client
        .delete_item()
        .table_name("duress_status")
        .key("user_id", user_id.into())
        .send()
        .await?;

    // TODO: Notify followers about duress cancellation (implement notification logic)

    Ok(json!({ "status": "Duress status canceled, followers notified" }))
}

// Function to enable testing mode
pub async fn enable_test_mode(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let client = get_dynamodb_client().await;

    let user_id = event.payload["pathParameters"]["user_id"].as_str().unwrap_or_default();

    info!("Testing mode enabled for user: {}", user_id);

    // Store testing mode activation in DynamoDB
    client
        .put_item()
        .table_name("test_mode")
        .item("user_id", user_id.into())
        .item("mode", "enabled".into())
        .item("timestamp", json!(chrono::Utc::now()).into())
        .send()
        .await?;

    Ok(json!({ "status": "Testing mode enabled for 5 minutes" }))
}

// Function to get map data for followed users
pub async fn get_map_data(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let client = get_dynamodb_client().await;

    let user_id = event.payload["pathParameters"]["user_id"].as_str().unwrap_or_default();

    // Query followed users and their last check-in data
    let result = client
        .query()
        .table_name("followed_users")
        .key_condition_expression("follower_id = :user_id")
        .expression_attribute_values(":user_id", user_id.into())
        .send()
        .await?;

    let followed_users: Vec<Value> = result
        .items
        .unwrap_or_default()
        .into_iter()
        .map(|item| json!({
            "user_id": item.get("user_id").unwrap().as_s().unwrap(),
            "username": item.get("username").unwrap().as_s().unwrap(),
            "location": item.get("location").unwrap().as_s().unwrap(),
            "duress": item.get("duress").unwrap().as_bool().unwrap(),
            "last_checkin": item.get("last_checkin").unwrap().as_s().unwrap(),
        }))
        .collect();

    Ok(json!({ "followed_users": followed_users }))
}

// Helper function to initialize DynamoDB client
async fn get_dynamodb_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}
