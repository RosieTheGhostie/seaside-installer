mod install;
mod uninstall;

pub use install::install;
pub use uninstall::uninstall;

const BINARY_RELEASE_NAME: &str = "x86_64-unknown-linux-gnu-seaside";
const BINARY_PATH: &str = "/usr/local/bin/seaside";
const CONFIG_DIRECTORY: &str = "~/.config/seaside";
const CONFIG_PATH: &str = "~/.config/seaside/Seaside.toml";
