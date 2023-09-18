use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;

use crate::{error::Error, models::user::User};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(post)));
}

#[derive(Deserialize, Debug)]
struct PostRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct PostResponse<'a> {
    pub message: &'a str,
}

async fn post(body: web::Json<PostRequest>) -> Result<HttpResponse, Error> {
    let user = User::find_by_username(&body.username);

    // If the user is already registered, we return an error.
    if user.is_ok() {
        return Err(Error {
            message: "This user is already registered".to_string(),
            status_code: 400,
        }
        .into());
    }

    // Encrypt the user password, we don't want to
    // store this password in plain-text.
    let mut hasher = Sha256::new();
    hasher.update(body.password.as_bytes());

    // we also can't store the password in raw bytes,
    // we have to transform it to a hex string
    let password = hex::encode(hasher.finalize());

    User::create(&body.username, &password)?;

    Ok(HttpResponse::Ok().json(json!({ "message": "success" })))
}
