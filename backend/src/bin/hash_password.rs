use password_hash::{PasswordHasher, SaltString};
use pbkdf2::{Pbkdf2, Params};
use rand_core::OsRng;


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

    password_hash
}