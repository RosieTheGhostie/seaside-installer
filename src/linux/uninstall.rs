use super::BINARY_PATH;
use crate::{
    cmd_args::UninstallArgs,
    common::{remove_dir_all, uninstall_config},
    info,
};

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

    remove_dir_all!(BINARY_PATH, repr: "binary")?;

    info!("successfully uninstalled binary");
    Ok(())
}
