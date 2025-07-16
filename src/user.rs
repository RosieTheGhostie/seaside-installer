use crate::debug;
use once_cell::sync::OnceCell;
use std::{path::Path, process::Command};

pub(super) static USER: OnceCell<String> = OnceCell::new();

pub fn user() -> &'static str {
    USER.get().expect("user was not initialized")
}

pub(super) fn get_sudo_user() -> Option<String> {
    std::env::var("SUDO_USER").ok()
}

pub fn transfer_ownership_to_user<P>(path: P, recursive: bool) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let user = user();
    debug!("transferring ownership of {:?} to {user}...", path.as_ref());
    let path_as_str = path.as_ref().as_os_str().to_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "path is not valid UTF-8")
    })?;
    let args = ["-hR", user, path_as_str];
    let output = Command::new("/bin/chown")
        .args(if recursive { &args[..] } else { &args[1..] })
        .output()?;
    if output.status.success() {
        debug!("successfully transferred ownership to {user}");
        Ok(())
    } else {
        Err(std::io::Error::other(
            "failed to transfer ownership of config directory",
        ))
    }
}

pub fn make_executable<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    debug!("marking {:?} as executable...", path.as_ref());
    let output = Command::new("/bin/chmod")
        .args([std::ffi::OsStr::new("u+x"), path.as_ref().as_os_str()])
        .output()?;
    if output.status.success() {
        debug!("successfully marked file as executable");
        Ok(())
    } else {
        Err(std::io::Error::other(
            "failed to mark file as executable",
        ))
    }
}
