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