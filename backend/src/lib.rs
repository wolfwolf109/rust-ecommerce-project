use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use core::panic;
use std::env;
use self::models::{NewUser, User};
use password_hash::{PasswordHasher, SaltString};
use pbkdf2::{Pbkdf2, Params};
use rand_core::OsRng;

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

pub fn hash_password(input_password: &str) -> String {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Configure PBKDF2 parameters
    let params = Params {
        rounds: 100_000,
        output_length: 32,
    };

    // Hash the password
    let password_hash = Pbkdf2.hash_password_customized(
        input_password.as_bytes(),
        None,
        None,
        params,
        &salt,
    ).unwrap().to_string();

    return password_hash;
}