use crate::{debug, info};
use windows_registry::Key;

const ENVIRONMENT: &str = "Environment";
const PATH: &str = "Path";
const DELIMITER: char = ';';

/// Appends `path` to the user's `PATH` environment variable.
pub fn add_to_path(path: &str) -> std::io::Result<()> {
    info!("adding {path:?} to PATH...");

    debug!("opening the environment registry key...");
    let key = open_user_environment_key()?;

    debug!("getting PATH environment variable...");
    let mut path_var = key.get_string(PATH).map_err(std::io::Error::other)?;
    if path_var
        .rsplit(DELIMITER) // using `rsplit` because it'll likely be near the end
        .any(|p| p == path)
    {
        info!("already in PATH. operation aborted");
        return Ok(());
    } else {
        debug!("{path:?} not in PATH. adding...");
    }

    if !path_var.ends_with(DELIMITER) {
        path_var.push(DELIMITER);
    }
    path_var.push_str(path);

    write_to_path_variable(&key, path_var)?;
    info!("successfully added {path:?} to PATH");
    Ok(())
}

/// Removes all instances of `path` from the user's `PATH` environment variable.
pub fn remove_from_path(path: &str) -> std::io::Result<()> {
    info!("removing {path:?} from PATH...");

    debug!("opening the environment registry key...");
    let key = open_user_environment_key()?;

    debug!("getting PATH environment variable...");
    let path_var = key.get_string(PATH).map_err(std::io::Error::other)?;
    let mut new_path_var = String::with_capacity(path_var.len() - path.len());
    let mut needs_delimiter = false;
    for p in path_var.split(DELIMITER).filter(|p| *p != path) {
        if needs_delimiter {
            new_path_var.push(DELIMITER);
        }
        new_path_var.push_str(p);
        needs_delimiter = true;
    }
    if path_var.len() == new_path_var.len() {
        info!("already not in PATH. operation aborted");
        return Ok(());
    } else {
        debug!("found in PATH. removing...");
    }

    write_to_path_variable(&key, new_path_var)?;
    info!("successfully removed {path:?} from PATH");
    Ok(())
}

/// Opens the user's environment registry key in read/write mode.
fn open_user_environment_key() -> std::io::Result<Key> {
    windows_registry::CURRENT_USER
        .options()
        .read()
        .write()
        .open(ENVIRONMENT)
        .map_err(std::io::Error::other)
}

/// Write `value` to the user's `PATH` environment variable.
fn write_to_path_variable(key: &Key, value: String) -> std::io::Result<()> {
    key.set_string(PATH, value).map_err(std::io::Error::other)
}
