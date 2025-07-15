use clap::{Args, Parser, Subcommand};
use semver::Version;

/// A simple installer, updater, and uninstaller for seaside.
#[derive(Clone, Debug, Parser)]
#[command(about, long_about = None)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub command: Command,
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

    /// Skips asking before replacing files.
    #[arg(short, long)]
    pub yes: bool,

    #[cfg(target_os = "windows")]
    /// The toolchain to use.
    #[arg(long, value_enum, default_value_t = Toolchain::Msvc)]
    pub toolchain: Toolchain,

    #[cfg(target_os = "windows")]
    /// Treat this installation as an update.
    ///
    /// This skips the steps that are only useful on a fresh install.
    #[arg(short, long)]
    pub update: bool,
}

#[derive(Args, Clone, Debug)]
pub struct UninstallArgs {
    /// Doesn't install the config file.
    #[arg(long, default_value_t = false)]
    pub keep_config: bool,
}

#[cfg(target_os = "windows")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, clap::ValueEnum)]
pub enum Toolchain {
    Msvc,
    Gnu,
}
