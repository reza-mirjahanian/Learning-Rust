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