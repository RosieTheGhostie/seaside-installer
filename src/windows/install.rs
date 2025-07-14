use super::{
    BINARY_PATH, BINARY_RELEASE_NAME_GNU, BINARY_RELEASE_NAME_MSVC, SEASIDE_PROGRAM_DATA,
    path::add_to_path,
};
use crate::{cmd_args::InstallArgs, common::*, debug, info, warn};
use reqwest::blocking::Client;

pub fn install(args: InstallArgs) -> std::io::Result<()> {
    info!("installing seaside...");

    let client = Client::builder().build().map_err(std::io::Error::other)?;

    let binary_exists = std::fs::exists(BINARY_PATH)?;
    if !(args.ask && binary_exists) || {
        warn!("a seaside binary is already present");
        ask("would you like to replace the existing binary?")?
    } {
        install_binary(&args, &client)?;
    }

    let config_path = get_config_path()?;
    let config_exists = std::fs::exists(&config_path)?;
    if !(args.ask && config_exists) || {
        warn!("a seaside config file is already present");
        ask("would you like to replace the existing config?")?
    } {
        install_config(&args, &client, &config_path)?;
    } else if config_exists && ask("would you like to update the config version to match?")? {
        update_config_version(&config_path, &args.version)?;
    }

    info!("install complete! :3");
    Ok(())
}

fn install_binary(args: &InstallArgs, client: &Client) -> std::io::Result<()> {
    info!("installing binary...");

    debug!("downloading binary from GitHub...");
    let binary_release_name = if args.use_gnu {
        BINARY_RELEASE_NAME_GNU
    } else {
        BINARY_RELEASE_NAME_MSVC
    };
    let bytes = client
        .get(generate_release_asset_url(
            &args.version,
            binary_release_name,
        ))
        .send()
        .map_err(std::io::Error::other)?
        .bytes()
        .map_err(std::io::Error::other)?;
    match std::fs::create_dir(SEASIDE_PROGRAM_DATA) {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(err) => return Err(err),
    }
    std::fs::write(BINARY_PATH, bytes)?;
    debug!("binary downloaded");

    if args.modify_path {
        add_to_path(SEASIDE_PROGRAM_DATA)?;
    }

    info!("successfully installed binary");
    Ok(())
}
