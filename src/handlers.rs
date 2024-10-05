use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Duration, Utc}; // Remove `Duration` since it's unused


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

pub async fn register_user(req: web::Json<RegisterRequest>) -> HttpResponse {
    if let Some(mut invite) = db::get_invite(&req.invite_code).await {
        invite.count += 1;
        if let Err(err) = db::update_invite(&invite).await {
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    } else {
        return HttpResponse::BadRequest().body("Invalid invite code");
    }

    let user_id = Uuid::new_v4().to_string();
    let user = db::User {
        id: user_id,
        invite_code: req.invite_code.clone(),
        normal_pin: req.normal_pin.clone(),
        duress_pin: req.duress_pin.clone(),
    };

    match db::save_user(&user).await {
        Ok(_) => HttpResponse::Ok().json(&user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn create_invite(req: web::Json<InviteRequest>) -> HttpResponse {
    let user_id = req.user_id.clone();

    // Fetch the user's invites within the past 168 hours (7 days)
    match db::get_user_invites(&user_id).await {
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
                count: 0,
                created_at: Utc::now(),     
            };

            match db::save_invite(&invite).await {
                Ok(_) => HttpResponse::Ok().json(InviteResponse { invite_code }),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
