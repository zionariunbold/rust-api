use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct Jwt;

impl Jwt {
    pub fn create_jwt(email: &str) -> String {
        dotenv().ok();
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .expect("Invalid time")
            .timestamp() as usize;

        let claims = Claims {
            sub: email.to_owned(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
        .expect("JWT creation failed")
    }

    pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        dotenv().ok();
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
}
