#![allow(dead_code, unused_variables)]

use auth_util::models;
use database::connect_to_database;

mod database{
    pub(crate) enum Status {
        Connected,
        Interrupted,
    }

    pub(crate) fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // get user from database
    }

}

mod auth_util {
    pub(crate) fn login(cred: models::Credentials) {
        // authenticate user
        crate::database::get_user();
    }

    fn logout() {
        // log out user
    }

    pub(crate) mod models{
        pub struct Credentials {
            username: String,
            password: String
        }
    }
}


pub fn authenticated(cred: models::Credentials)  { // auth_util::models::Credentials
    if database::Status::Connected = connect_to_database() { // database::connect_to_database
         auth_util::login(cred);
    }
}

