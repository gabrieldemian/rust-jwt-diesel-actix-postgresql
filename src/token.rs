//
// This file handles everything about JWT token.
// Generation, validation, and the JWT Middleware
// that is used on authorized routes
//
use futures_util::future::LocalBoxFuture;
use std::{
    env,
    future::{ready, Ready},
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::Error as ActixError,
    error::ErrorUnauthorized,
    http::{header, StatusCode},
    HttpMessage,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// subject (user id)
    sub: String,
    /// expires
    exp: usize,
    /// issued at
    iat: usize,
}

#[derive(Clone)]
pub struct Token;

impl Token {
    /// If the secret is not set on `.env`, default to "secret"
    fn get_secret() -> Vec<u8> {
        env::var("JWT_SECRET")
            .unwrap_or("secret".to_owned())
            .as_bytes()
            .to_vec()
    }
    /// Authenticate and generate a new token given an `id`.
    /// Returns the token if successfull
    pub fn authenticate(id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();

        // issued at
        let iat = now.timestamp() as usize;

        // token expiration
        // if there is no expiration the .env file, default to 20 minutes
        let exp = (now
            + env::var("JWT_EXPIRATION_MINUTES")
                .map(|x| Duration::minutes(x.parse::<i64>().unwrap()))
                .unwrap_or_else(|_| Duration::minutes(20)))
        .timestamp() as usize;

        let my_claims = Claims {
            sub: id.to_string(),
            iat,
            exp,
        };

        let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS256,
            ..Default::default()
        };

        let token = encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(&Self::get_secret()),
        )?;

        Ok(token)
    }

    /// Check if the token is valid
    /// It will decode the token, and return a boolean if it's valid or not
    pub fn validate(token: &str) -> bool {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(&Self::get_secret()),
            &Validation::new(Algorithm::HS512),
        )
        .is_ok()
    }
}

/// Middleware that handles JWT authentication.
/// We will extract the token on the Authorization header,
/// and validate it. If it's ok we proceed to the endpoint,
/// if not we throw an error with UNAUTHORIZED.
#[derive(Clone, Debug)]
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

// This is where the middleware actually happens,
// actix-web has this indirection of requiring 2 structs
// to fully implement the middleware
impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        // validation of the JWT token starts here
        Box::pin(async move {
            let res = fut.await?;

            // Extract token from the authentication header
            let token = match res.request().headers().get(header::AUTHORIZATION) {
                Some(v) => v,
                None => {
                    let json_error = crate::error::Error {
                        status_code: StatusCode::UNAUTHORIZED.into(),
                        message: "Authorization header not present".to_string(),
                    };
                    return Err(ErrorUnauthorized(json_error));
                }
            };

            // Transform token to a string
            let token = match std::str::from_utf8(token.as_bytes()) {
                Ok(v) => v,
                Err(_) => {
                    let json_error = crate::error::Error {
                        status_code: StatusCode::UNAUTHORIZED.into(),
                        message: "Token is not a valid string".to_string(),
                    };
                    return Err(ErrorUnauthorized(json_error));
                }
            };

            // Remove the prefix "Bearer ", we only want the token
            let token = token.replace("Bearer ", "");

            // Token is a valid string at this point
            // try to decode and validate the token
            let claims = match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(&Token::get_secret()),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(c) => c.claims,
                Err(e) => {
                    let message = match e.kind() {
                        ErrorKind::ExpiredSignature => {
                            "This token has expired, you must login again."
                        }
                        _ => "Invalid token",
                    }
                    .to_owned();

                    let json_error = crate::error::Error {
                        status_code: StatusCode::UNAUTHORIZED.into(),
                        message,
                    };

                    return Err(ErrorUnauthorized(json_error));
                }
            };

            // Here, the token is valid, we can call the endpoint
            res.request().extensions_mut().insert(claims.sub.clone());

            Ok(res)
        })
    }
}
