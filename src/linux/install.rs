use super::{BINARY_PATH, BINARY_RELEASE_NAME};
use crate::{
    cmd_args::InstallArgs,
    common::{
        ask, generate_release_asset_url, get_config_path, install_config, update_config_version,
    },
    debug, info, warn,
};
use reqwest::blocking::Client;

pub fn install(args: InstallArgs) -> std::io::Result<()> {
    info!("installing seaside...");

    let client = Client::builder().build().map_err(std::io::Error::other)?;

    let binary_exists = std::fs::exists(BINARY_PATH)?;
    if !binary_exists || args.yes || {
        warn!("a seaside binary is already present");
        ask("would you like to replace the existing binary?")?
    } {
        install_binary(&args, &client)?;
    }

    let config_path = get_config_path()?;
    let config_exists = std::fs::exists(&config_path)?;
    if !config_exists || args.yes || {
        warn!("a seaside config file is already present");
        ask("would you like to replace the existing config?")?
    } {
        install_config(&args, &client, &config_path)?;
    } else if config_exists
        && (args.yes || ask("would you like to update the config version to match?")?)
    {
        update_config_version(&config_path, &args.version)?;
    }

    info!("install complete! :3");
    Ok(())
}

fn install_binary(args: &InstallArgs, client: &Client) -> std::io::Result<()> {
    info!("installing binary...");

    debug!("downloading binary from GitHub...");
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
    debug!("binary downloaded");

    info!("successfully installed binary");
    Ok(())
}
