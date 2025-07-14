use super::BINARY_PATH;
use crate::{cmd_args::UninstallArgs, common::*, debug, info};

pub fn uninstall(args: UninstallArgs) -> std::io::Result<()> {
    uninstall_binary(&args)?;
    if !args.keep_config {
        uninstall_config(&args)
    } else {
        Ok(())
    }
}

fn uninstall_binary(_args: &UninstallArgs) -> std::io::Result<()> {
    info!("uninstalling binary...");
    match std::fs::remove_file(BINARY_PATH) {
        Ok(()) => {
            info!("successfully uninstalled binary");
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            info!("binary was not present");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
