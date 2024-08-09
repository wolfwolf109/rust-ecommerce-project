use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::http::hyper::server::conn;
use core::panic;
use std::env;
use self::models::{NewUser, User};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, user_type: &str, username: &str, email: &str, password_hash: &str) -> User {
    use schema::users;

    let new_user = NewUser {
        user_type,
        username,
        email,
        password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}