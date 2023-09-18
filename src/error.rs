use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Error {
    pub status_code: u16,
    pub message: String,
}

impl Error {
    pub fn new(status_code: u16, message: String) -> Error {
        Error {
            status_code,
            message,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

// Convert Diesel errors to Error
// With this we can use the quotation mark `?` on endpoint handlers.
impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        match error {
            DieselError::DatabaseError(_, err) => Error::new(409, err.message().to_string()),
            DieselError::NotFound => Error::new(404, "Record not found".to_string()),
            err => Error::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
