use semver::Version;
use std::{fmt::Display, path::PathBuf};

pub const GITHUB_REPO: &str = "https://github.com/RosieTheGhostie/seaside";
pub const CONFIG_NAME: &str = "Seaside.toml";

pub fn ask<S>(message: S) -> std::io::Result<bool>
where
    S: Display,
{
    loop {
        print!("{message} (y/n) > ");
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp)?;
        temp.make_ascii_lowercase();
        match temp.strip_suffix('\n').unwrap_or(&temp) {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => eprintln!("invalid response. please try again"),
        }
    }
}

pub fn get_config_path() -> std::io::Result<PathBuf> {
    get_config_dir().map(|directory| directory.join(CONFIG_NAME))
}

pub fn get_config_dir() -> std::io::Result<PathBuf> {
    match directories::ProjectDirs::from("", "", "seaside") {
        Some(project_directories) => Ok(project_directories.config_dir().to_path_buf()),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "couldn't find a home directory",
        )),
    }
}

pub fn generate_release_asset_url(version: &Version, asset_name: &str) -> String {
    format!("{GITHUB_REPO}/releases/download/v{version}/{asset_name}")
}
