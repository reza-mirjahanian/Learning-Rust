#![allow(dead_code, unused_variables)]
mod database; //why
mod auth_utils;


use auth_utils::models::Credentials;
use database::Status;






pub fn authenticated(cred: Credentials)  { // auth_util::models::Credentials
    if Status::Connected = database::connect_to_database() { // database::connect_to_database
         auth_utils::login(cred);
    }
}

