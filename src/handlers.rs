use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc}; // You may leave `Duration` in case it's used for time-based invite restrictions
use aws_sdk_dynamodb::Client;
use tracing::{info, error};

use crate::db;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    invite_code: String,
    normal_pin: String,
    duress_pin: String,
}

#[derive(Debug, Deserialize)]
pub struct InviteRequest {
    user_id: String, // ID of the user generating the invite
}

#[derive(Debug, Serialize)]
pub struct InviteResponse {
    invite_code: String,
}

fn log_error_chain(error: &dyn std::error::Error) {
    let mut current_error = Some(error);
    while let Some(err) = current_error {
        error!("{}", err);
        current_error = err.source();
    }
}


pub async fn register_user(
    client: web::Data<Client>, // Access the DynamoDB client from the app state
    req: web::Json<RegisterRequest>
) -> HttpResponse {
    // Get the invite from DynamoDB using the provided invite code
    info!("Received register request: {:?}", req);
    match db::get_invite(&client, &req.invite_code).await {
        Ok(Some(mut invite)) => {
            invite.invite_count += 1;
            if let Err(err) = db::update_invite(&client, &invite).await {
                error!("Failed to update invite: {:?}", err);
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        },
        Ok(None) => {
            error!("Invalid invite code provided: {}", req.invite_code);
            return HttpResponse::BadRequest().body("Invalid invite code");
        },
        Err(err) => {
            error!("Failed to fetch invite from DynamoDB: {:?}", err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }

    let user_id = Uuid::new_v4().to_string();
    let user = db::User {
        id: user_id.clone(),
        invite_code: req.invite_code.clone(),
        normal_pin: req.normal_pin.clone(),
        duress_pin: req.duress_pin.clone(),
    };

    match db::save_user(&client, &user).await {
        Ok(_) =>{
            info!("Successfully registered user: {}", user_id);
            HttpResponse::Ok().json(&user)},
        Err(err) => {
            error!("Failed to save user to DynamoDB: {:?}", err);

            
            HttpResponse::InternalServerError().body(err.to_string())},
    }
}

pub async fn create_invite(
    client: web::Data<Client>, // Access the DynamoDB client from the app state
    req: web::Json<InviteRequest>
) -> HttpResponse {
    let user_id = req.user_id.clone();

    // Fetch the user's invites within the past 168 hours (7 days)
    match db::get_user_invites(&client, &user_id).await {
        Ok(invites) => {
            let recent_invites: Vec<_> = invites
                .iter()
                .filter(|invite| Utc::now() - invite.created_at < Duration::hours(168))
                .collect();

            if recent_invites.len() >= 5 {
                return HttpResponse::BadRequest().body("Invite limit exceeded for the last 168 hours");
            }

            // Generate a unique invite code
            let invite_code = Uuid::new_v4().to_string();

            let invite = db::Invite {
                code: invite_code.clone(),
                invitor_id: user_id,
                invite_count: 0,
                created_at: Utc::now(),
            };

            match db::save_invite(&client, &invite).await {
                Ok(_) => HttpResponse::Ok().json(InviteResponse { invite_code }),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
