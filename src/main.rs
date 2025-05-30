mod models;
mod repositories;
mod rocket_routes;
mod schema;
mod utils;

use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::get("/")]
pub async fn get_index() -> &'static str {
    "Hello world"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/api",
            rocket::routes![
                rocket_routes::user::get_users,
                rocket_routes::user::create_user,
                rocket_routes::user::delete_user,
                rocket_routes::user::update_user,
                rocket_routes::auth::login,
                get_index
            ],
        )
        .attach(DbConn::init())
        .launch()
        .await;
}
