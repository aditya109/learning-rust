use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}