use super::{SEASIDE_PROGRAM_DATA, path::remove_from_path};
use crate::{cmd_args::UninstallArgs, common::*, info};

pub fn uninstall(args: UninstallArgs) -> std::io::Result<()> {
    info!("uninstalling seaside...");

    uninstall_binary(&args)?;
    if !args.keep_config {
        uninstall_config(&args)?;
    }

    info!("uninstall complete! :3");
    Ok(())
}

fn uninstall_binary(_args: &UninstallArgs) -> std::io::Result<()> {
    info!("uninstalling binary...");

    remove_dir_all!(SEASIDE_PROGRAM_DATA, repr: "binary")?;
    remove_from_path(SEASIDE_PROGRAM_DATA)?;

    info!("successfully uninstalled binary");
    Ok(())
}
