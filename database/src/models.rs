use diesel::prelude::*;
use serde::Serialize;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: Option<i32>,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name= crate::schema::notes)]
#[derive(Serialize)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub note_name: String,
    pub note_tag: String,
    pub note_data: String,
}

#[derive(Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name= crate::schema::notes)]
pub struct NewNote<'a> {
    pub user_id: i32,
    pub note_name: &'a str,
    pub note_tag: &'a str,
    pub note_data: &'a str,
}
