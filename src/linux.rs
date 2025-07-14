use crate::{
    cmd_args::{InstallArgs, UninstallArgs},
    common::*,
};
use log::{info, trace};
use reqwest::blocking::Client;
use std::path::Path;

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
