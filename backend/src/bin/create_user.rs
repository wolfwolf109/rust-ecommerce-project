use std::io::stdin;
use backend::*;


fn main() {
    let connection = &mut establish_connection();

    let mut user_type = String::new();
    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    println!("Enter your user type [customer, admin, superadmin]: ");
    stdin().read_line(&mut user_type).unwrap();
    let user_type = user_type.trim_end();

    println!("Enter your username: ");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    println!("Enter your email: ");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end();

    println!("Enter your password: ");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();

    let password_hash = hash_password(password);

    let user = create_user(connection, user_type, username, email, &password_hash);
    println!("\nCreated User {} with id {}", username, user.id)

}