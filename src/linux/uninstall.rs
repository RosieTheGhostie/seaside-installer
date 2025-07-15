use super::{BINARY_PATH, CONFIG_DIRECTORY};
use crate::{cmd_args::UninstallArgs, common::uninstall_config};

pub fn uninstall(args: UninstallArgs) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248muninstalling seaside...\x1b[0m");

    uninstall_binary()?;
    if !args.keep_config {
        uninstall_config(CONFIG_DIRECTORY)?;
    }

    eprintln!("\x1b[38;5;248muninstall complete! :3\x1b[0m");
    Ok(())
}

fn uninstall_binary() -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248muninstalling binary...\x1b[0m");

    match std::fs::remove_file(BINARY_PATH) {
        Ok(()) => {
            eprintln!("\x1b[38;5;248msuccessfully removed binary\x1b[0m");
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("\x1b[38;5;248mbinary was not present\x1b[0m");
        }
        Err(err) => return Err(err),
    }

    eprintln!("\x1b[38;5;248msuccessfully uninstalled binary\x1b[0m");
    Ok(())
}
