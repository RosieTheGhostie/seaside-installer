use super::{
    BINARY_DIRECTORY, BINARY_PATH, BINARY_RELEASE_NAME_GNU, BINARY_RELEASE_NAME_MSVC, CONFIG_PATH,
    path::add_to_path,
};
use crate::{
    cmd_args::{InstallArgs, Toolchain},
    common::{
        ask, generate_release_asset_url, install_config, try_create_parent, update_config_version,
    },
    debug,
};

pub fn install(args: InstallArgs) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248minstalling seaside...\x1b[0m");

    let binary_exists = std::fs::exists(BINARY_PATH)?;
    if !binary_exists || args.yes || {
        eprintln!("\x1b[33m[WARNING] a seaside binary is already present\x1b[0m");
        ask("would you like to replace the existing binary?")?
    } {
        install_binary(&args)?;
    }

    let config_exists = std::fs::exists(CONFIG_PATH)?;
    if !config_exists || args.yes || {
        eprintln!("\x1b[33m[WARNING] a seaside config file is already present\x1b[0m");
        ask("would you like to replace the existing config?")?
    } {
        install_config(&args, CONFIG_PATH)?;
    } else if config_exists
        && (args.yes || ask("would you like to update the config version to match?")?)
    {
        update_config_version(CONFIG_PATH, &args.version)?;
    }

    eprintln!("\x1b[38;5;248minstall complete! :3\x1b[0m");
    Ok(())
}

fn install_binary(args: &InstallArgs) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248minstalling binary...\x1b[0m");

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
        add_to_path(BINARY_DIRECTORY)?;
    }

    eprintln!("\x1b[38;5;248msuccessfully installed binary\x1b[0m");
    Ok(())
}
