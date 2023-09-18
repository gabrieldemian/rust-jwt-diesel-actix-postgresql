use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{error::Error, models::notes::model::Note, token::Auth};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/notes")
            .wrap(Auth)
            .route(web::get().to(get))
            .route(web::post().to(post)),
    )
    .service(
        web::resource("/notes/{id}")
            .wrap(Auth)
            .route(web::put().to(put))
            .route(web::delete().to(delete))
            .route(web::get().to(get_one)),
    );
}

async fn get() -> Result<HttpResponse, Error> {
    let notes = Note::find_all()?;

    Ok(HttpResponse::Ok().json(json!({
        "notes": notes,
    })))
}

#[derive(Deserialize, Debug)]
struct PostRequest {
    title: String,
    content: String,
}

async fn post(body: web::Json<PostRequest>) -> Result<HttpResponse, Error> {
    let note = Note::create(&body.title, &body.content)?;
    Ok(HttpResponse::Ok().json(note))
}

#[derive(Deserialize, Debug)]
struct PutRequest {
    title: String,
    content: Option<String>,
}

async fn put(path: web::Path<i32>, body: web::Json<PutRequest>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let note = Note::update(id, &body.title, body.content.as_ref().map(|x| x.as_str()))?;

    Ok(HttpResponse::Ok().json(note))
}

async fn delete(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    Note::delete(id)?;

    Ok(HttpResponse::Ok().json(json!({ "message": "success" })))
}

async fn get_one(path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let note = Note::find(id)?;

    Ok(HttpResponse::Ok().json(json!({ "note": note })))
}
