use self::models::*;
use diesel::prelude::*;
use backend::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results = users.limit(5).select(User::as_select()).load(connection).expect("Error showing users");

    println!("Displaying {} users", results.len());
    println!("----------------I'm a line----------------");
    for user in results {
        println!("id: {}", user.id);
        println!("Username: {}", user.username);
        println!("Email: {}", user.email);
        println!("Created_at: {}", user.created_at);
        println!("User Type: {}", user.user_type);
        println!("----------------I'm a line----------------");
    }
}