mod install;
mod uninstall;

pub use install::*;
pub use uninstall::*;

use core::fmt::Display;
use std::{io::Write, path::PathBuf};

/// The name of seaside's configuration file.
pub const CONFIG_NAME: &str = "Seaside.toml";

/// Asks a yes/no question to the user and returns their answer.
pub fn ask<S>(message: S) -> std::io::Result<bool>
where
    S: Display,
{
    #[cfg(not(target_os = "windows"))]
    const NEWLINE: &str = "\n";
    #[cfg(target_os = "windows")]
    const NEWLINE: &str = "\r\n";
    loop {
        print!("{message} (y/n) > ");
        std::io::stdout().flush()?;
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp)?;
        temp.make_ascii_lowercase();
        match temp.strip_suffix(NEWLINE).unwrap_or(&temp) {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => eprintln!("invalid response. please try again"),
        }
    }
}

/// Returns the path at which seaside's configuration file should reside.
pub fn get_config_path() -> std::io::Result<PathBuf> {
    get_config_dir().map(|directory| directory.join(CONFIG_NAME))
}

/// Returns the directory in which seaside's configuration file should reside.
pub fn get_config_dir() -> std::io::Result<PathBuf> {
    match directories::ProjectDirs::from("", "", "seaside") {
        Some(project_directories) => Ok(project_directories.config_dir().to_path_buf()),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "couldn't find a home directory",
        )),
    }
}
