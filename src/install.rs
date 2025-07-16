use crate::{
    ask,
    cmd_args::InstallArgs,
    consts::{BINARY_PATH, CONFIG_NAME},
    debug, get_config,
};
use regex::Regex;
use semver::Version;
use std::{borrow::Cow, io::BufRead, path::Path};

pub fn install(args: InstallArgs) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248minstalling seaside...\x1b[0m");

    let binary_exists = std::fs::exists(BINARY_PATH)?;
    if !binary_exists || args.yes || {
        eprintln!("\x1b[33m[WARNING] a seaside binary is already present\x1b[0m");
        ask("would you like to replace the existing binary?")?
    } {
        install_binary(&args)?;
    }

    let config_path = get_config::path()?;
    let config_exists = std::fs::exists(&config_path)?;
    if !config_exists || args.yes || {
        eprintln!("\x1b[33m[WARNING] a seaside config file is already present\x1b[0m");
        ask("would you like to replace the existing config?")?
    } {
        install_config(&args, &config_path)?;
    } else if config_exists
        && (args.yes || ask("would you like to update the config version to match?")?)
    {
        update_config_version(&config_path, &args.version)?;
    }

    eprintln!("\x1b[38;5;248minstall complete! :3\x1b[0m");
    Ok(())
}

fn install_binary(args: &InstallArgs) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248minstalling binary...\x1b[0m");

    debug!("downloading binary from GitHub...");
    #[cfg(target_os = "linux")]
    let request_builder = ureq::get(generate_release_asset_url(
        &args.version,
        crate::consts::BINARY_RELEASE_NAME,
    ));
    #[cfg(target_os = "windows")]
    let request_builder = {
        let binary_release_name = match args.toolchain {
            crate::cmd_args::Toolchain::Msvc => crate::consts::BINARY_RELEASE_NAME_MSVC,
            crate::cmd_args::Toolchain::Gnu => crate::consts::BINARY_RELEASE_NAME_GNU,
        };
        ureq::get(generate_release_asset_url(
            &args.version,
            binary_release_name,
        ))
    };
    let mut response_body = request_builder
        .call()
        .map_err(std::io::Error::other)?
        .into_body();
    #[cfg(target_os = "windows")]
    try_create_dir(crate::consts::BINARY_DIRECTORY)?;
    let mut file = std::fs::File::create(BINARY_PATH)?;
    std::io::copy(&mut response_body.as_reader(), &mut file)?;
    debug!("binary downloaded");

    #[cfg(target_os = "windows")]
    if !args.update {
        crate::windows_path::add_to_path(crate::consts::BINARY_DIRECTORY)?;
    }

    eprintln!("\x1b[38;5;248msuccessfully installed binary\x1b[0m");
    Ok(())
}

/// Installs the default configuration file.
fn install_config(args: &InstallArgs, path: &Path) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248minstalling config...\x1b[0m");

    debug!("downloading config from GitHub...");
    let mut response_body = ureq::get(generate_release_asset_url(&args.version, CONFIG_NAME))
        .call()
        .map_err(std::io::Error::other)?
        .into_body();
    let parent = path.parent().unwrap();
    try_create_dir(parent)?;
    let mut file = std::fs::File::create(path)?;
    std::io::copy(&mut response_body.as_reader(), &mut file)?;
    debug!("config downloaded");

    #[cfg(target_os = "linux")]
    {
        let user = crate::user::user();
        debug!("transferring ownership of {parent:?} to {user}...");
        let parent_as_str = parent.as_os_str().to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "path is not valid UTF-8")
        })?;
        let output = std::process::Command::new("/bin/chown")
            .args(["-hR", user, parent_as_str])
            .output()?;
        if output.status.success() {
            debug!("successfully transferred ownership to {user}");
        } else {
            return Err(std::io::Error::other(
                "failed to transfer ownership of config directory",
            ));
        }
    }

    eprintln!("\x1b[38;5;248msuccessfully installed config\x1b[0m");
    Ok(())
}

/// Updates a configuration file's target version of seaside to `version`.
fn update_config_version(path: &Path, version: &Version) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    let replace_pattern = format!(r#"version = "{version}"$COMMENT"#);

    {
        let version_regex =
            Regex::new(r#"^[ \t]*version[ \t]*=[ \t]*".*"(?<COMMENT>[ \t]*(?:#.*)?)$"#).unwrap();
        debug!("opening config file...");
        let file = std::fs::File::open(path)?;
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
    eprintln!("\x1b[38;5;248msuccessfully updated config version\x1b[0m");
    Ok(())
}

/// Tries to create `directory`.
///
/// This will not throw an error if `directory` already exists.
fn try_create_dir<P>(directory: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    debug!("attempting to create {:?}", directory.as_ref());
    match std::fs::create_dir(directory) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(err) => Err(err),
    }
}

/// Generates a URL from which one may download a given version of an asset.
fn generate_release_asset_url(version: &Version, asset_name: &str) -> String {
    format!("https://github.com/RosieTheGhostie/seaside/releases/download/v{version}/{asset_name}")
}
