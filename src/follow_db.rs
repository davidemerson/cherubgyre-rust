use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Serialize, Deserialize};
use tracing::{info, error};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Follow {
	pub follower_id: String,
	pub followed_id: String,
}

// Adds a new follow relationship to the DynamoDB "Follow" table
pub async fn add_follow(
	client: &Client,
	follower_id: &str,
	followed_id: &str,
) -> Result<(), Error> {
	info!("Adding a follow relationship in DynamoDB");

	client
		.put_item()
		.table_name("Follow")
		.item("id", AttributeValue::S(Uuid::new_v4().to_string()))
		.item("follower_id", AttributeValue::S(follower_id.to_string()))
		.item("followed_id", AttributeValue::S(followed_id.to_string()))
		.send()
		.await?;

	Ok(())
}

// Removes a follow relationship from the DynamoDB "Follow" table
pub async fn remove_follow(
	client: &Client,
	follower_id: &str,
	followed_id: &str,
) -> Result<(), Error> {
	info!("Removing a follow relationship in DynamoDB");

	client
		.delete_item()
		.table_name("Follow")
		.key("follower_id", AttributeValue::S(follower_id.to_string()))
		.key("followed_id", AttributeValue::S(followed_id.to_string()))
		.send()
		.await?;

	Ok(())
}

// Retrieves all follows for a given follower_id
pub async fn get_follows(client: &Client, followed_id: &str) -> Result<Vec<Follow>, Error> {
	info!("Fetching follows for a given followed_id");

	let result = client
		.scan()
		.table_name("Follow")
		.filter_expression("followed_id = :followed_id")
		.expression_attribute_values(":followed_id", AttributeValue::S(followed_id.to_string()))
		.send()
		.await?;

	let follows = result
		.items
		.unwrap_or_default()
		.into_iter()
		.map(|item| Follow {
			followed_id: item
				.get("followed_id")
				.and_then(|v| v.as_s().ok())
				.map(|s| s.to_string())
				.unwrap_or_else(String::new),
			follower_id: item
				.get("follower`1   _id")
				.and_then(|v| v.as_s().ok())
				.map(|s| s.to_string())
				.unwrap_or_else(String::new),
		})
		.collect();

	Ok(follows)
}
