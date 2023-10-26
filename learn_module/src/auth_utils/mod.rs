pub(crate) fn login(cred: models::Credentials) {
    // authenticate user
    crate::database::get_user();
}

fn logout() {
    // log out user
}

pub(crate) mod models;

// there is new way, create auth_utils.rs file at the root level. move mod.rs content to the auth_utils.rs