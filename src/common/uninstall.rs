use super::get_config_dir;
use crate::{cmd_args::UninstallArgs, debug, info};

/// Uninstalls seaside's global configuration file.
pub fn uninstall_config(_args: &UninstallArgs) -> std::io::Result<()> {
    info!("uninstalling config...");
    debug!("getting config directory...");
    let directory = get_config_dir()?;
    debug!("got config directory");
    remove_dir_all!(directory, repr: "config")?;
    info!("successfully uninstalled config");
    Ok(())
}

/// Tries to remove the given directory and all of its descendants.
///
/// This will not throw an error if `directory` does not exist.
macro_rules! remove_dir_all {
    ($directory:expr, repr: $repr:literal) => {
        match ::std::fs::remove_dir_all($directory) {
            ::core::result::Result::Ok(()) => {
                $crate::info!("successfully removed {}", $repr);
                ::core::result::Result::Ok(())
            }
            ::core::result::Result::Err(err) if err.kind() == ::std::io::ErrorKind::NotFound => {
                $crate::info!("{} was not present", $repr);
                ::core::result::Result::Ok(())
            }
            ::core::result::Result::Err(err) => ::core::result::Result::Err(err),
        }
    };
}
pub(crate) use remove_dir_all;
