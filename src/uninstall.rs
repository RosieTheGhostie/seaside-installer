use crate::{cmd_args::UninstallArgs, get_config};
use std::path::Path;

pub fn uninstall(args: UninstallArgs) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248muninstalling seaside...\x1b[0m");

    uninstall_binary()?;
    if !args.keep_config {
        uninstall_config()?;
    }

    eprintln!("\x1b[38;5;248muninstall complete! :3\x1b[0m");
    Ok(())
}

fn uninstall_binary() -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248muninstalling binary...\x1b[0m");

    #[cfg(target_os = "linux")]
    match std::fs::remove_file(crate::consts::BINARY_PATH) {
        Ok(()) => {
            eprintln!("\x1b[38;5;248msuccessfully removed binary\x1b[0m");
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("\x1b[38;5;248mbinary was not present\x1b[0m");
        }
        Err(err) => return Err(err),
    }
    #[cfg(target_os = "windows")]
    {
        remove_dir_all(crate::consts::BINARY_DIRECTORY, "binary")?;
        crate::windows_path::remove_from_path(crate::consts::BINARY_DIRECTORY)?;
    }

    eprintln!("\x1b[38;5;248msuccessfully uninstalled binary\x1b[0m");
    Ok(())
}

/// Uninstalls seaside's global configuration file.
fn uninstall_config() -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248muninstalling config...\x1b[0m");

    remove_dir_all(get_config::dir()?, "config")?;

    eprintln!("\x1b[38;5;248msuccessfully uninstalled config\x1b[0m");
    Ok(())
}

/// Tries to remove the given directory and all of its descendants.
///
/// This will not throw an error if `directory` does not exist.
fn remove_dir_all<P>(directory: P, repr: &'static str) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    match std::fs::remove_dir_all(directory) {
        Ok(()) => {
            eprintln!("\x1b[38;5;248msuccessfully removed {repr}\x1b[0m");
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("\x1b[38;5;248m{repr} was not present\x1b[0m");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
