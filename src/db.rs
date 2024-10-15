use aws_config::load_from_env;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

pub async fn get_dynamodb_client() -> Client {
    // Set up the region provider
    let region_provider = RegionProviderChain::default_provider().or_else("eu-north-1");

    // Load the AWS configuration
    let config = aws_config::from_env().region(region_provider).load().await;

    // Create DynamoDB client from the configuration
    Client::new(&config)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub invite_code: String,
    pub normal_pin: String,
    pub duress_pin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub code: String,
    pub invitor_id: String,			// ID of the user who created the invite
    pub invite_count: u32,			// Number of invitees registered using this invite
    pub created_at: DateTime<Utc>,	// Date when invite was created
}

pub async fn save_user(client: &Client, user: &User) -> Result<(), Error> {
    info!("here i am");
    client
		.put_item()
        .table_name("User")
        .item("id", AttributeValue::S(user.id.clone()))
        .item("invite_code", AttributeValue::S(user.invite_code.clone()))
        .item("normal_pin", AttributeValue::S(user.normal_pin.clone()))
        .item("duress_pin", AttributeValue::S(user.duress_pin.clone()))
        .send()
        .await?;
    Ok(())
}

pub async fn save_invite(client: &Client, invite: &Invite) -> Result<(), Error> {
    info!("here i am");
    client
		.put_item()
        .table_name("Invite")
        .item("code", AttributeValue::S(invite.code.clone()))
        .item("invitor_id", AttributeValue::S(invite.invitor_id.clone()))
        .item(
        	"invite_count", 
        	AttributeValue::N(invite.invite_count.to_string()),
        )
        .item(
        	"created_at", 
        	AttributeValue::S(invite.created_at.to_rfc3339()),
        )
        .send()
        .await?;

    Ok(())
}

pub async fn get_invite(client: &Client, code: &str) -> Result<Option<Invite>, Error> {
    let result = client
    	.get_item()
        .table_name("Invite")
        .key("code", AttributeValue::S(code.to_string()))
        .send()
        .await?;

    if let Some(item) = result.item {
        let invite = Invite {
            code: item
				.get("code")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string())
                .unwrap_or_else(String::new),
            invitor_id: item
				.get("invitor_id")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string())
                .unwrap_or_else(String::new),
            invite_count: item
				.get("invite_count")
                .and_then(|v| v.as_n().ok())
                .and_then(|n| n.parse::<u32>().ok())
                .unwrap_or(0),
            created_at: item
				.get("created_at")
                .and_then(|v| v.as_s().ok())
                .map(|v| DateTime::parse_from_rfc3339(v).unwrap().with_timezone(&Utc))
                .unwrap_or(Utc::now()),
        };
        Ok(Some(invite))
    } else {
        Ok(None)
    }
}

pub async fn get_user_invites(client: &Client, user_id: &str) -> Result<Vec<Invite>, Error> {
    let result = client
		.scan()
        .table_name("Invite")
        .filter_expression("invitor_id = :id")
        .expression_attribute_values(":id", AttributeValue::S(user_id.to_string()))
        .send()
        .await?;

    let invites = result
		.items
		.unwrap_or_default()
        .into_iter()
        .map(|item| Invite {
            code: item
            	.get("code")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string())
                .unwrap_or_else(String::new),
            invitor_id: item
				.get("invitor_id")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string())
                .unwrap_or_else(String::new),
            invite_count: item
            	.get("invite_count")
                .and_then(|v| v.as_n().ok())
                .and_then(|n| n.parse::<u32>().ok())
                .unwrap_or(0),
            created_at: item
            	.get("created_at")
                .and_then(|v| v.as_s().ok())
                .map(|v| DateTime::parse_from_rfc3339(v).unwrap().with_timezone(&Utc))
                .unwrap_or(Utc::now()),
        })
        .collect();

    Ok(invites)
}

pub async fn update_invite(client: &Client, invite: &Invite) -> Result<(), Error> {
    client
    	.update_item()
        .table_name("Invite")
        .key("code", AttributeValue::S(invite.code.clone()))
        .update_expression(
			"SET invitor_id = :invitor_id, invite_count = :invite_count, created_at = :created_at",
        )
        .expression_attribute_values(":invitor_id", AttributeValue::S(invite.invitor_id.clone()))
        .expression_attribute_values(
        	":invite_count", 
        	AttributeValue::N(invite.invite_count.to_string()),
        )
        .expression_attribute_values(
        	":created_at", 
        	AttributeValue::S(invite.created_at.to_rfc3339()),
        )
        .send()
        .await?;

    Ok(())
}
