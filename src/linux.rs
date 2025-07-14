use crate::{
    cmd_args::{InstallArgs, UninstallArgs},
    common::{CONFIG_NAME, ask, generate_release_asset_url, get_config_dir, get_config_path},
};
use log::{info, trace};
use regex::Regex;
use reqwest::blocking::Client;
use semver::Version;
use std::{borrow::Cow, io::BufRead, path::Path};

const BINARY_RELEASE_NAME: &str = "x86_64-unknown-linux-gnu-seaside";
const BINARY_PATH: &str = "/usr/local/bin/seaside";

pub fn install(args: InstallArgs) -> std::io::Result<()> {
    let client = Client::builder().build().map_err(std::io::Error::other)?;
    if !args.ask
        || (std::fs::exists(BINARY_PATH)? && ask("would you like to replace the existing binary?")?)
    {
        install_binary(&args, &client)?;
    }
    let config_path = get_config_path()?;
    let config_exists = std::fs::exists(&config_path)?;
    if !args.ask || (config_exists && ask("would you like to replace the existing config?")?) {
        install_config(&args, &client, &config_path)?;
    } else if config_exists && ask("would you like to update the config version to match?")? {
        update_config_version(&config_path, &args.version)?;
    }
    Ok(())
}

fn install_binary(args: &InstallArgs, client: &Client) -> std::io::Result<()> {
    info!("installing binary...");
    trace!("downloading binary from GitHub...");
    let bytes = client
        .get(generate_release_asset_url(
            &args.version,
            BINARY_RELEASE_NAME,
        ))
        .send()
        .map_err(std::io::Error::other)?
        .bytes()
        .map_err(std::io::Error::other)?;
    std::fs::write(BINARY_PATH, bytes)?;
    trace!("binary downloaded");
    info!("successfully installed binary");
    Ok(())
}

fn install_config<P>(args: &InstallArgs, client: &Client, path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    info!("installing config...");
    trace!("downloading config from GitHub...");
    let bytes = client
        .get(generate_release_asset_url(&args.version, CONFIG_NAME))
        .send()
        .map_err(std::io::Error::other)?
        .bytes()
        .map_err(std::io::Error::other)?;
    std::fs::write(path, bytes)?;
    trace!("config downloaded");
    info!("successfully installed config");
    Ok(())
}

fn update_config_version<P>(path: P, version: &Version) -> std::io::Result<()>
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

pub fn uninstall(args: UninstallArgs) -> std::io::Result<()> {
    uninstall_binary(&args)?;
    if !args.keep_config {
        uninstall_config(&args)
    } else {
        Ok(())
    }
}

fn uninstall_binary(_args: &UninstallArgs) -> std::io::Result<()> {
    info!("uninstalling binary...");
    match std::fs::remove_file(BINARY_PATH) {
        Ok(()) => {
            info!("successfully uninstalled binary");
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            info!("binary was not present");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn uninstall_config(_args: &UninstallArgs) -> std::io::Result<()> {
    info!("uninstalling config...");
    trace!("getting config directory...");
    let directory = get_config_dir()?;
    trace!("got config directory");
    match std::fs::remove_dir_all(directory) {
        Ok(()) => {
            info!("successfully uninstalled config");
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            info!("config was not present");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
