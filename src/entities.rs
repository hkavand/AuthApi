extern crate argonautica;
use rocket::{serde::{Deserialize, Serialize}, request::{FromRequest, self, Outcome}, http::Header};
use rocket::response::Responder;
use crate::utils::*;
extern crate dotenv;

#[derive(Deserialize,Debug)]
#[serde(crate = "rocket::serde")]
pub struct RegisterUserRequest<'r> {
    pub email: &'r str,
    pub password: &'r str,
    pub name: &'r str,
    pub phonenumber: &'r str
}

#[derive(Debug, Serialize, Responder)]
pub struct LoginResponse{
    pub jwt: String
}

#[derive(Debug, Serialize, Responder)]
pub struct ResponseError {
    pub Error: String
}

#[derive(Debug, Serialize, Responder)]
pub struct ResponseOk {
    pub Msg: String
}

#[derive(Responder)]
pub struct MyResponder<T> {
    inner: T,
    my_header: Header<'static>,
}
impl<'r, 'o: 'r, T: Responder<'r, 'o>> MyResponder<T> {
    pub fn new(inner: T, token: String) -> Self {
        MyResponder {
            inner,
            my_header: Header::new("token", token),
        }
    }
}

pub struct Token(Option<String>);

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

impl Token {
    pub fn get_token (&self) -> Option<String>{
        let Token(token) = self;
        token.to_owned()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => {
                let bearer_token: Vec<&str> = token.split(" ").collect();
                let real_token = bearer_token[1];
                
                match jwt::verify(real_token){
                    Some(_) => Outcome::Success(Token(Some(real_token.to_string()))),
                    None => Outcome::Success(Token(None))
                }
            }
            None => Outcome::Success(Token(None)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emailtoken {
    pub email: String
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String, 
    pub phonenumber: String,
    pub fullname: String
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'r> {
    pub email: &'r str,
    pub password: &'r str                                                                                                                                                                                                                                                           
}