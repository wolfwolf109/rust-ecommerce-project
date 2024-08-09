use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_type: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub user_type: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}