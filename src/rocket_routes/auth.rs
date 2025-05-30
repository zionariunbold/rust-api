use crate::repositories::user::UserRepository;
use crate::utils::hash::Hash;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::serde::json::json;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

use crate::utils::jwt::Jwt;
use rocket::async_trait;
use rocket::{
    Request,
    request::{FromRequest, Outcome},
};

use crate::DbConn;

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<login>")]
pub async fn login(
    mut db: Connection<DbConn>,
    login: Json<UserLogin>,
) -> Result<Value, Custom<Value>> {
    match UserRepository::find_by_email(&mut db, &login.email).await {
        Ok(user) if Hash::verify_password(&login.password, &user.password) => {
            let token = crate::utils::jwt::Jwt::create_jwt(&login.email);
            Ok(json!({ "token": token }))
        }
        _ => Err(Custom(
            Status::Unauthorized,
            json!({"status": "error","message":"Invalid credentials"}),
        )),
    }
}

pub struct AuthenticatedUser {
    pub _email: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req.headers().get_one("Authorization");

        if let Some(token) = token {
            let token = token.trim_start_matches("Bearer ");
            if let Ok(claims) = Jwt::validate_jwt(token) {
                return Outcome::Success(AuthenticatedUser { _email: claims.sub });
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
