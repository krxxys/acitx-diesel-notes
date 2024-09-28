use diesel::prelude::*;
use serde::Serialize; 

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct User {
    pub id: i32, 
    pub username: String, 
    pub password: String
}


#[derive(Insertable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: Option<i32>,
    pub username: &'a str, 
    pub password: &'a str,
}