extern crate argonautica;
use crate::entities::*;
extern crate dotenv;

pub mod password {
    use argonautica::Hasher;
    use argonautica::Verifier;
    use rand::{distributions::Alphanumeric, Rng};

    pub fn create_hash_and_salt (password: & str) -> (String, String) {
        let mut hasher = Hasher::default();
        
        let salt: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let hash = hasher
            .with_password(password)
            .with_secret_key(&salt)
            .hash()
            .unwrap();

        return (salt, hash)
    }

    pub fn verify (password: &str, salt: &str, hash: &str) -> bool {
        let mut verifier = Verifier::default();
        verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(salt)
        .verify()
        .unwrap()
    } 
}

pub mod jwt {
    use std::{ fs::{File}, io::{BufWriter, Write, Read}};
    use jwt_simple::prelude::{HS256Key, MACLike};
    use dotenv::*;

    use super::Emailtoken;

    pub fn get_key () -> HS256Key{
        dotenv().ok();
        let res = File::open("Key.txt");
        match res {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                let key = HS256Key::from_bytes(&buffer);
                key
            },
            Err(_) => {
                let key = HS256Key::generate();
                let key_bytes = key.to_bytes();
                let f = File::create("Key.txt").expect("couldnt create key file");
                let mut f = BufWriter::new(f);
                f.write_all(&key_bytes).expect("Unable to write data");
                key
            }
        }
    }

    pub fn verify (token: &str) -> Option<String> {
        let key = get_key();
        match key.verify_token::<Emailtoken>(token, None) {
            Ok(claim) => Some(claim.custom.email),
            Err(_) => None
        }
    }
}

pub mod validation {
    use phonenumber::country::Id::IR;
    use fancy_regex::Regex;
    use phonenumber;

    use super::RegisterUserRequest;

    pub fn validate_new_user (user: &RegisterUserRequest) -> Result<(), String>{
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
        let phone = phonenumber::parse(Some(IR), user.phonenumber).unwrap();
        let pass_regex = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();

        if !email_regex.is_match(user.email).unwrap(){
            Err("email is not the correct format".to_string())
        } else if !phone.is_valid() {
            Err("phone number is not in the correct format".to_string())
        } else if !pass_regex.is_match(user.password).unwrap(){
            Err("the password must meet security policy (at least 8 characters, including one alphabet letter and one number)".to_string())
        } else{
            Ok(())
        }
    }
}