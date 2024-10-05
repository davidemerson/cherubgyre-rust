// duress_handlers.rs
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::duress_db::{self,UserPreferences};

#[derive(Debug, Deserialize)]
pub struct DuressRequest {
    duress_type: String,
    message: String,
    timestamp: String,
    additional_data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct CancelDuressRequest {
    normal_pin: String,
    confirm: bool,
}

#[derive(Debug, Serialize)]
pub struct MapInfo {
    pub user_id: String,
    pub username: String,
    pub location: String,
    pub duress: bool,
    pub last_checkin: String,
}



// POST /users/{user_id}/duress
pub async fn trigger_duress(path: web::Path<String>, req: web::Json<DuressRequest>) -> HttpResponse {
    let user_id = path.into_inner();

    // Placeholder: Notify followers and nearby users
    // TODO: Integrate with actual notification and location service
    duress_db::log_duress_event(&user_id, &req.duress_type, &req.message, &req.timestamp).await;

    HttpResponse::Ok().body("Duress notification triggered")
}

// POST /users/{user_id}/duress/cancel
pub async fn cancel_duress(path: web::Path<String>, req: web::Json<CancelDuressRequest>) -> HttpResponse {
    let user_id = path.into_inner();

    if req.confirm {
        // Placeholder: Validate normal_pin
        duress_db::cancel_duress(&user_id).await;

        HttpResponse::Ok().body("Duress notification canceled")
    } else {
        HttpResponse::BadRequest().body("Confirmation required to cancel duress")
    }
}

// POST /users/{user_id}/test-mode
pub async fn enable_test_mode(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();

    // Placeholder: Enable test mode for 5 minutes
    duress_db::enable_test_mode(&user_id).await;

    HttpResponse::Ok().body("Test mode enabled for 5 minutes")
}

// GET /users/{user_id}/map
pub async fn get_map_info(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();

    // Placeholder: Retrieve location and duress status of all followed users
    match duress_db::get_followed_users_map_info(&user_id).await {
        Ok(map_info) => HttpResponse::Ok().json(map_info),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// GET /users/{user_id}/preferences
pub async fn get_preferences(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();

    match duress_db::get_user_preferences(&user_id).await {
        Ok(preferences) => HttpResponse::Ok().json(preferences),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// PATCH /users/{user_id}/preferences
pub async fn update_preferences(path: web::Path<String>, req: web::Json<UserPreferences>) -> HttpResponse {
    let user_id = path.into_inner();

    match duress_db::update_user_preferences(&user_id, req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Preferences updated"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

