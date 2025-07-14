use super::{BINARY_PATH, BINARY_RELEASE_NAME};
use crate::{cmd_args::InstallArgs, common::*, debug, info};
use reqwest::blocking::Client;

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
