use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use crate::entities::*;
use crate::repo::*;
use crate::utils::*;
use rocket::response::status;

use jwt_simple::prelude::*;

#[post("/register", data = "<user>")]
pub fn register(user: Json<RegisterUserRequest<'_>>) -> Result<Json<ResponseOk>, status::Custom<Json<ResponseError>>> {
    let user = user.into_inner();
    match validation::validate_new_user(&user) {
        Ok(_) => {
            match user_repo::get_by_email(user.email){
                Ok(_) => Err(status::Custom(Status::BadRequest, Json(ResponseError { Error: "a user with the same email exists!!".to_string() }))),
                Err(_) => {
                    let res = user_repo::insert(user);
                    match res {
                        Ok(_) => Ok(Json(ResponseOk{ Msg: "congrats: user added, sign in please".to_string()})),
                        Err(_) => Err(status::Custom(Status::BadRequest, Json(ResponseError { Error: "something BAD happened, couldn't register user".to_string() })))
                    }
                }
            }
        },
        Err(msg) => Err(status::Custom(Status::BadRequest, Json(ResponseError { Error: msg })))
    }
}

#[post("/login", data = "<req>")]
pub fn login(req: Json<LoginRequest>) -> Result<MyResponder<Json<LoginResponse>>, status::Custom<Json<ResponseError>>>{
    match user_repo::get_by_email(req.email){
        Ok(user) => {
            if !password::verify(req.password, &user.passwordsalt, &user.passwordhash){
                Err(status::Custom(Status::BadRequest, Json(ResponseError { Error: "wrong username or password".to_string() })))
            } else {
                let key = jwt::get_key();
                let claims = Claims::with_custom_claims(Emailtoken{email: req.email.to_string()}, Duration::from_hours(2));
                let token = key.authenticate(claims).unwrap();

                Ok(MyResponder::new(Json(LoginResponse { jwt: token.clone() }), token))
            }
        },
        Err(_) => Err(status::Custom(Status::BadRequest, Json(ResponseError { Error: "no user found ".to_string() })))
    }
}

#[get("/info")]
pub fn get_info(token: crate::entities::Token) -> Result<Json<UserResponse>, status::Custom<Json<ResponseError>>> {
    match token.get_token() {
        None => Err(Custom(Status::Forbidden, Json(ResponseError { Error: "You need to be logged in".to_string() }))),
        Some(token) => {
            let email = jwt::verify(&token).unwrap();
            match user_repo::get_by_email(&email){
                Ok(user) => Ok(Json(UserResponse { id: user.id, email: user.emailaddress, phonenumber: user.phonennumber, fullname: user.fullname })),
                Err(_) => Err(Custom(Status::InternalServerError, Json(ResponseError {Error: "Couldn't fetch user from database".to_string()})))
            }
        }
    }
}