// follow_handlers.rs
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::follow_db;

#[derive(Debug, Deserialize)]
pub struct FollowRequest {
    user_id: String, // ID of the user to follow or unfollow
}


// POST /users/{user_id}/follow
pub async fn follow_user(path: web::Path<String>, req: web::Json<FollowRequest>) -> HttpResponse {
    let follower_id = path.into_inner();
    let followed_id = req.user_id.clone();

    match follow_db::add_follow(&follower_id, &followed_id).await {
        Ok(_) => HttpResponse::Ok().body("Followed successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// POST /users/{user_id}/unfollow
pub async fn unfollow_user(path: web::Path<String>, req: web::Json<FollowRequest>) -> HttpResponse {
    let follower_id = path.into_inner();
    let followed_id = req.user_id.clone();

    match follow_db::remove_follow(&follower_id, &followed_id).await {
        Ok(_) => HttpResponse::Ok().body("Unfollowed successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// GET /users/{user_id}/followers
pub async fn get_followers(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();

    match follow_db::get_followers(&user_id).await {
        Ok(followers) => HttpResponse::Ok().json(followers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// DELETE /users/{user_id}/followers/{follower_id}
pub async fn delete_follower(path: web::Path<(String, String)>) -> HttpResponse {
    let (followed_id, follower_id) = path.into_inner();

    match follow_db::remove_follow(&follower_id, &followed_id).await {
        Ok(_) => HttpResponse::Ok().body("Follower deleted successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
