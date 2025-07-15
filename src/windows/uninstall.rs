use super::{BINARY_DIRECTORY, CONFIG_DIRECTORY, path::remove_from_path};
use crate::{
    cmd_args::UninstallArgs,
    common::{remove_dir_all, uninstall_config},
};

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

    remove_dir_all(BINARY_DIRECTORY, "binary")?;
    remove_from_path(BINARY_DIRECTORY)?;

    eprintln!("\x1b[38;5;248msuccessfully uninstalled binary\x1b[0m");
    Ok(())
}
