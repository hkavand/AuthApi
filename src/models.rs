use crate::schema::*;
use serde::Serialize;
use chrono::prelude::*;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub emailaddress: String,
    pub passwordhash: String,
    pub passwordsalt: String,
    pub fullname: String,
    pub phonennumber: String
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser<'r> {
    pub emailaddress: &'r str,
    pub passwordhash: &'r str,
    pub passwordsalt: &'r str,
    pub fullname: &'r str,
    pub phonennumber: &'r str
}

#[derive(Debug, Queryable, Serialize)]
pub struct UserToken {
    pub id: i32,
    pub userid: i32,
    pub token: String,
    pub createdat: chrono::NaiveDateTime,
    pub expiresat: chrono::NaiveDateTime
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="user_tokens"]
pub struct NewUserToken<'r> {
    pub userid: i32,
    pub token: &'r str,
    pub createdat: NaiveDateTime,
    pub expiresat: NaiveDateTime
}