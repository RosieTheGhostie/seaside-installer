use log::{info, trace};
use regex::Regex;
use semver::Version;
use std::{
    borrow::Cow,
    fmt::Display,
    io::{BufRead, Write},
    path::{Path, PathBuf},
};

const GITHUB_REPO: &str = "https://github.com/RosieTheGhostie/seaside";

pub const CONFIG_NAME: &str = "Seaside.toml";

pub fn ask<S>(message: S) -> std::io::Result<bool>
where
    S: Display,
{
    loop {
        print!("{message} (y/n) > ");
        std::io::stdout().flush()?;
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

pub fn update_config_version<P>(path: P, version: &Version) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut buffer = Vec::new();
    let replace_pattern = format!(r#"version = "{version}"$COMMENT"#);

    {
        let version_regex =
            Regex::new(r#"^[ \t]*version[ \t]*=[ \t]*".*"(?<COMMENT>[ \t]*(?:#.*)?)$"#).unwrap();
        trace!("opening config file...");
        let file = std::fs::File::open(&path)?;
        trace!("opened config file");
        let mut lines_iter = std::io::BufReader::new(file).lines().map_while(Result::ok);

        trace!("finding and replacing version...");
        for line in lines_iter.by_ref() {
            let replaced = version_regex.replace(&line, &replace_pattern);
            buffer.extend(replaced.bytes());
            buffer.push(b'\n');
            if matches!(replaced, Cow::Owned(_)) {
                trace!("version replaced");
                break;
            }
        }

        trace!("writing remainder of file to buffer...");
        for line in lines_iter {
            buffer.extend(line.bytes());
            buffer.push(b'\n');
        }
        trace!("entire file written to buffer");
    }

    trace!("writing buffer back to file...");
    std::fs::write(path, buffer)?;
    trace!("buffer written to file");
    info!("successfully updated config version");
    Ok(())
}
