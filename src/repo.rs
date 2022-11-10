pub mod user_repo {

    use crate::entities::*;
    use crate::models::*;
    use crate::schema::*;
    use crate::utils::*;
    use diesel::*;

    pub fn insert(user: RegisterUserRequest<'_>) -> Result<usize, diesel::result::Error>{
        let (salt, hash) = password::create_hash_and_salt(user.password);
        insert_into(users::table).values(
            NewUser{
                emailaddress: user.email,
                passwordhash: &hash,
                passwordsalt: &salt,
                fullname: user.name,
                phonennumber: user.phonenumber
            }
        )
        .execute(&crate::establish_connection())
    }

    pub fn get_by_email (email: &str) -> Result<User, diesel::result::Error> {
        users::table.select(users::all_columns)
            .filter(users::emailaddress.eq(email))
            .first::<User>(&crate::establish_connection())
            
    }

    pub fn get_by_id (id: i32) -> Result<User, diesel::result::Error> {
        users::table.select(users::all_columns)
            .filter(users::id.eq(id))
            .first::<User>(&crate::establish_connection())
    }
}

pub mod usertoken_repo {
    use crate::models::*;
    use crate::schema::*;
    use diesel::*;

    pub fn insert (data: NewUserToken<'_>) -> Result<usize, diesel::result::Error> {
        insert_into(user_tokens::table).values(data).execute(&crate::establish_connection())
    }

    pub fn get_by_token(token: &str) -> Result<UserToken, diesel::result::Error> {
        user_tokens::table.select(user_tokens::all_columns)
            .filter(user_tokens::token.eq(token))
            .first::<UserToken>(&crate::establish_connection())
    }
}