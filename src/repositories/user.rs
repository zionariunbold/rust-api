use crate::models::user::{NewUser, PublicUser, User};
use crate::schema::*;
use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct UserRepository;

impl UserRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(
        c: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<PublicUser>> {
        users::table
            .select((users::id, users::name, users::email)) // Explicitly select only public fields
            .limit(limit)
            .load::<PublicUser>(c)
            .await
    }
    pub async fn find_by_email(c: &mut AsyncPgConnection, user_email: &str) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(user_email))
            .first::<User>(c)
            .await
    }

    pub async fn create_user(
        c: &mut AsyncPgConnection,
        mut new_user: NewUser,
    ) -> QueryResult<User> {
        if let Ok(hashed) = hash(&new_user.password, DEFAULT_COST) {
            new_user.password = hashed;
        } else {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, user: User) -> QueryResult<User> {
        let hashed_password = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");

        diesel::update(users::table.find(id))
            .set((
                users::name.eq(user.name),
                users::email.eq(user.email),
                users::password.eq(hashed_password),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}
