use once_cell::sync::OnceCell;

pub(super) static USER: OnceCell<String> = OnceCell::new();

pub fn user() -> &'static str {
    USER.get().expect("user was not initialized")
}

pub(super) fn get_sudo_user() -> Option<String> {
    std::env::var("SUDO_USER").ok()
}
