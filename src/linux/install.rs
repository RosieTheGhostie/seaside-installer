use super::{BINARY_PATH, BINARY_RELEASE_NAME};
use crate::{
    cmd_args::InstallArgs,
    common::{
        ask, generate_release_asset_url, get_config_path, install_config, try_create_parent,
        update_config_version,
    },
    debug, info, warn,
};

pub fn install(args: InstallArgs) -> std::io::Result<()> {
    info!("installing seaside...");

    let binary_exists = std::fs::exists(BINARY_PATH)?;
    if !binary_exists || args.yes || {
        warn!("a seaside binary is already present");
        ask("would you like to replace the existing binary?")?
    } {
        install_binary(&args)?;
    }

    let config_path = get_config_path()?;
    let config_exists = std::fs::exists(&config_path)?;
    if !config_exists || args.yes || {
        warn!("a seaside config file is already present");
        ask("would you like to replace the existing config?")?
    } {
        install_config(&args, &config_path)?;
    } else if config_exists
        && (args.yes || ask("would you like to update the config version to match?")?)
    {
        update_config_version(&config_path, &args.version)?;
    }

    info!("install complete! :3");
    Ok(())
}

fn install_binary(args: &InstallArgs) -> std::io::Result<()> {
    info!("installing binary...");

    debug!("downloading binary from GitHub...");
    let mut response_body = ureq::get(generate_release_asset_url(
        &args.version,
        BINARY_RELEASE_NAME,
    ))
    .call()
    .map_err(std::io::Error::other)?
    .into_body();
    try_create_parent(BINARY_PATH)?;
    let mut file = std::fs::File::create(BINARY_PATH)?;
    std::io::copy(&mut response_body.as_reader(), &mut file)?;
    debug!("binary downloaded");

    info!("successfully installed binary");
    Ok(())
}
