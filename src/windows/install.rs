use super::{
    BINARY_PATH, BINARY_RELEASE_NAME_GNU, BINARY_RELEASE_NAME_MSVC, SEASIDE_PROGRAM_DATA,
    path::add_to_path,
};
use crate::{
    cmd_args::{InstallArgs, Toolchain},
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
    let binary_release_name = match args.toolchain {
        Toolchain::Msvc => BINARY_RELEASE_NAME_MSVC,
        Toolchain::Gnu => BINARY_RELEASE_NAME_GNU,
    };
    let mut response_body = ureq::get(generate_release_asset_url(
        &args.version,
        binary_release_name,
    ))
    .call()
    .map_err(std::io::Error::other)?
    .into_body();
    try_create_parent(BINARY_PATH)?;
    let mut file = std::fs::File::create(BINARY_PATH)?;
    std::io::copy(&mut response_body.as_reader(), &mut file)?;
    debug!("binary downloaded");

    if !args.update {
        add_to_path(SEASIDE_PROGRAM_DATA)?;
    }

    info!("successfully installed binary");
    Ok(())
}
