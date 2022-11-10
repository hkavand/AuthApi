use rocket::serde::json::Json;
use crate::entities::*;
use crate::repo::*;
use crate::utils::*;
use rocket::response::status::{self, BadRequest};

use jwt_simple::prelude::*;

#[post("/register", data = "<user>")]
pub fn register(user: Json<RegisterUserRequest<'_>>) -> Result<String, status::BadRequest<Json<ResponseError>>> {
    let user = user.into_inner();
    match validation::validate_new_user(&user) {
        Ok(_) => {
            match user_repo::get_by_email(user.email){
                Ok(_) => Err(status::BadRequest(Some(Json(ResponseError { Error: "a user with the same email exists!!".to_string() })))),
                Err(_) => {
                    let res = user_repo::insert(user);
                    match res {
                        Ok(_) => Ok("congrats: user added, sign in please".to_string()),
                        Err(_) => Err(status::BadRequest(Some(Json(ResponseError { Error: "something BAD happened, couldn't register user".to_string() }))))
                    }
                }
            }
        },
        Err(msg) => Err(status::BadRequest(Some(Json(ResponseError { Error: msg }))))
    }
}

#[post("/login", data = "<req>")]
pub fn login(req: Json<LoginRequest>) -> Result<MyResponder<Json<LoginResponse>>, status::BadRequest<Json<ResponseError>>>{
    match user_repo::get_by_email(req.email){
        Ok(user) => {
            if !password::verify(req.password, &user.passwordsalt, &user.passwordhash){
                Err(status::BadRequest(Some (Json(ResponseError { Error: "wrong username or password".to_string() }))))
            } else {
                let key = jwt::get_key();
                let claims = Claims::with_custom_claims(Emailtoken{email: req.email.to_string()}, Duration::from_hours(2));
                let token = key.authenticate(claims).unwrap();

                Ok(MyResponder::new(Json(LoginResponse { jwt: token.clone() }), token))
            }
        },
        Err(_) => Err(status::BadRequest(Some(Json(ResponseError { Error: "no user found ".to_string() }))))
    }
}

#[get("/info")]
pub fn get_info(token: crate::entities::Token) -> Result<Json<UserResponse>, status::BadRequest<Json<ResponseError>>> {
    let email = jwt::verify(&token.get_token()).unwrap();

    match user_repo::get_by_email(&email){
        Ok(user) => Ok(Json(UserResponse { id: user.id, email: user.emailaddress, phonenumber: user.phonennumber, fullname: user.fullname })),
        Err(_) => Err(BadRequest(Some(Json(ResponseError {Error: "Couldn't fetch user from database".to_string()}))))
    }
}