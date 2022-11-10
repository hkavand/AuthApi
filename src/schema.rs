// @generated automatically by Diesel CLI.

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        userid -> Int4,
        token -> Varchar,
        createdat -> Timestamp,
        expiresat -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        emailaddress -> Varchar,
        passwordhash -> Varchar,
        passwordsalt -> Varchar,
        fullname -> Varchar,
        phonennumber -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user_tokens,
    users,
);
