use crate::debug;
use windows_registry::Key;

const ENVIRONMENT: &str = "Environment";
const PATH: &str = "Path";
const DELIMITER: char = ';';

/// Appends `path` to the user's `PATH` environment variable.
pub fn add_to_path(path: &str) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248madding {path:?} to PATH...\x1b[0m");

    debug!("opening the environment registry key...");
    let key = open_user_environment_key()?;

    debug!("getting PATH environment variable...");
    let mut path_var = key.get_string(PATH).map_err(std::io::Error::other)?;
    if path_var
        .rsplit(DELIMITER) // using `rsplit` because it'll likely be near the end
        .any(|p| p == path)
    {
        eprintln!("\x1b[38;5;248malready in PATH. operation aborted\x1b[0m");
        return Ok(());
    } else {
        debug!("{path:?} not in PATH. adding...");
    }

    if !path_var.ends_with(DELIMITER) {
        path_var.push(DELIMITER);
    }
    path_var.push_str(path);

    write_to_path_variable(&key, path_var)?;
    eprintln!("\x1b[38;5;248msuccessfully added {path:?} to PATH\x1b[0m");
    Ok(())
}

/// Removes all instances of `path` from the user's `PATH` environment variable.
pub fn remove_from_path(path: &str) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248mremoving {path:?} from PATH...\x1b[0m");

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
        eprintln!("\x1b[38;5;248malready not in PATH. operation aborted\x1b[0m");
        return Ok(());
    } else {
        debug!("found in PATH. removing...");
    }

    write_to_path_variable(&key, new_path_var)?;
    eprintln!("\x1b[38;5;248msuccessfully removed {path:?} from PATH\x1b[0m");
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
