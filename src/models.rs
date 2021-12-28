use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool
}