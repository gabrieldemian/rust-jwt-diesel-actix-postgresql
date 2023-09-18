use actix_web::{http::StatusCode, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;

use crate::{error::Error, models::user::User, token::Token};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(post)));
}

#[derive(Deserialize, Debug)]
struct PostRequest {
    username: String,
    password: String,
}

async fn post(body: web::Json<PostRequest>) -> Result<HttpResponse, Error> {
    // Check that this user exists
    let user = User::find_by_username(&body.username)?;

    // We will take the provided password,
    // transform to sha2 hex string,
    // and check if it matches the string on the database
    let mut hasher = Sha256::new();
    hasher.update(body.password.as_bytes());

    let password = hex::encode(hasher.finalize());

    // Check that the password is correct
    if password != user.password {
        return Err(Error {
            status_code: StatusCode::UNAUTHORIZED.into(),
            message: "Wrong username or password".to_owned(),
        }
        .into());
    }

    // Generate JWT token
    let token = Token::authenticate(user.id).unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "message": "success",
        "token": token,
    })))
}
