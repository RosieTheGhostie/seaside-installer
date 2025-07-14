use super::CONFIG_NAME;
use crate::{cmd_args::InstallArgs, debug, info};
use regex::Regex;
use reqwest::blocking::Client;
use semver::Version;
use std::{borrow::Cow, io::BufRead, path::Path};

/// The URL of the seaside GitHub repository.
const GITHUB_REPO: &str = "https://github.com/RosieTheGhostie/seaside";

/// Generates a URL from which one may download a given version of an asset.
pub fn generate_release_asset_url(version: &Version, asset_name: &str) -> String {
    format!("{GITHUB_REPO}/releases/download/v{version}/{asset_name}")
}

/// Installs the default configuration file at `path`.
pub fn install_config<P>(args: &InstallArgs, client: &Client, path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    info!("installing config...");
    debug!("downloading config from GitHub...");
    let bytes = client
        .get(generate_release_asset_url(&args.version, CONFIG_NAME))
        .send()
        .map_err(std::io::Error::other)?
        .bytes()
        .map_err(std::io::Error::other)?;
    try_create_parent(&path)?;
    std::fs::write(path, bytes)?;
    debug!("config downloaded");
    info!("successfully installed config");
    Ok(())
}

/// Updates a configuration file's target version of seaside to `version`.
pub fn update_config_version<P>(path: P, version: &Version) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut buffer = Vec::new();
    let replace_pattern = format!(r#"version = "{version}"$COMMENT"#);

    {
        let version_regex =
            Regex::new(r#"^[ \t]*version[ \t]*=[ \t]*".*"(?<COMMENT>[ \t]*(?:#.*)?)$"#).unwrap();
        debug!("opening config file...");
        let file = std::fs::File::open(&path)?;
        debug!("opened config file");
        let mut lines_iter = std::io::BufReader::new(file).lines().map_while(Result::ok);

        debug!("finding and replacing version...");
        for line in lines_iter.by_ref() {
            let replaced = version_regex.replace(&line, &replace_pattern);
            buffer.extend(replaced.bytes());
            buffer.push(b'\n');
            if matches!(replaced, Cow::Owned(_)) {
                debug!("version replaced");
                break;
            }
        }

        debug!("writing remainder of file to buffer...");
        for line in lines_iter {
            buffer.extend(line.bytes());
            buffer.push(b'\n');
        }
        debug!("entire file written to buffer");
    }

    debug!("writing buffer back to file...");
    std::fs::write(path, buffer)?;
    debug!("buffer written to file");
    info!("successfully updated config version");
    Ok(())
}

/// Tries to create a parent directory for `path`.
///
/// This will not throw an error if `path` does not have a parent nor if its parent already exists.
pub fn try_create_parent<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let parent = match path.as_ref().parent() {
        Some(parent) => parent,
        None => return Ok(()),
    };
    match std::fs::create_dir(parent) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(err),
    }
}
