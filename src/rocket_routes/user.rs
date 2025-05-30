use crate::models::user::{NewUser, User};
use crate::repositories::user::UserRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::serde::json::json;
use rocket_db_pools::Connection;

use crate::DbConn;

use super::auth::AuthenticatedUser;

#[rocket::get("/users")]
pub async fn get_users(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    UserRepository::find_multiple(&mut db, 100)
        .await
        .map(|users| json!(users))
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
}

#[rocket::post("/users", format = "json", data = "<new_user>")]
pub async fn create_user(
    mut db: Connection<DbConn>,
    new_user: Json<NewUser>,
) -> Result<Value, Custom<Value>> {
    match UserRepository::find_by_email(&mut db, &new_user.email).await {
        Ok(_) => Err(Custom(
            Status::Conflict,
            json!({"message": "И-мэйл аль хэдийн бүртгэгдсэн байна"}),
        )),
        Err(diesel::result::Error::NotFound) => {
            UserRepository::create_user(&mut db, new_user.into_inner())
                .await
                .map(|user| json!(user))
                .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
        }
        Err(e) => Err(Custom(
            Status::InternalServerError,
            json!({"message": format!("Алдаа гарлаа: {}", e)}),
        )),
    }
}

#[rocket::put("/users/<id>", format = "json", data = "<a_user>")]
pub async fn update_user(
    mut db: Connection<DbConn>,
    id: i32,
    _user: AuthenticatedUser,
    a_user: Json<User>,
) -> Result<Value, Custom<Value>> {
    match UserRepository::find(&mut db, id).await {
        Ok(_) => match UserRepository::update(&mut db, id, a_user.into_inner()).await {
            Ok(updated_user) => Ok(json!({
                "status": "success",
                "message": "Хэрэглэгчийн мэдээллийг шинэчиллээ",
                "user": updated_user
            })),
            Err(e) => Err(Custom(
                Status::InternalServerError,
                json!({"message": format!("Шинэчлэхэд алдаа гарлаа: {}", e)}),
            )),
        },
        Err(diesel::result::Error::NotFound) => Err(Custom(
            Status::NotFound,
            json!({"status": "error", "message": "Хэрэглэгч олдсонгүй"}),
        )),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            json!({"message": format!("Алдаа гарлаа: {}", e)}),
        )),
    }
}

#[rocket::delete("/users/<id>")]
pub async fn delete_user(
    mut db: Connection<DbConn>,
    _user: AuthenticatedUser,
    id: i32,
) -> Result<Value, Custom<Value>> {
    match UserRepository::find(&mut db, id).await {
        Ok(_) => match UserRepository::delete(&mut db, id).await {
            Ok(_) => {
                Ok(json!({"status": "success", "message": "Хэрэглэгчийн мэдээллийг устгалаа"}))
            }
            Err(e) => Err(Custom(
                Status::InternalServerError,
                json!({"message": format!("Устгахад алдаа гарлаа: {}", e)}),
            )),
        },
        Err(diesel::result::Error::NotFound) => Err(Custom(
            Status::NotFound,
            json!({"status": "error", "message": "Хэрэглэгч олдсонгүй"}),
        )),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            json!({"message": format!("Алдаа гарлаа: {}", e)}),
        )),
    }
}
