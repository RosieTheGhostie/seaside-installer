use crate::logging::LogLevel;
use clap::{Args, Parser, Subcommand};
use semver::Version;

#[derive(Clone, Debug, Parser)]
#[command(about, long_about = None)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub command: Command,

    /// Set the log level.
    #[arg(long)]
    pub log: Option<LogLevel>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Installs or updates seaside.
    Install(InstallArgs),
    /// Uninstalls an existing installation.
    Uninstall(UninstallArgs),
}

#[derive(Args, Clone, Debug)]
pub struct InstallArgs {
    /// The version of seaside to install.
    pub version: Version,

    /// Ask before replacing any existing files.
    #[arg(short, long, default_value_t = true)]
    pub ask: bool,

    #[cfg_attr(target_os = "windows", doc = r"Use the GNU toolchain instead of MSVC.")]
    #[cfg_attr(target_os = "windows", arg(long, default_value_t = false))]
    pub use_gnu: bool,

    #[cfg_attr(
        target_os = "windows",
        doc = r"Try to add seaside to the PATH environment variable."
    )]
    #[cfg_attr(target_os = "windows", arg(long, default_value_t = true))]
    pub modify_path: bool,
}

#[derive(Args, Clone, Debug)]
pub struct UninstallArgs {
    /// Doesn't install the config file.
    #[arg(long, default_value_t = false)]
    pub keep_config: bool,
}
