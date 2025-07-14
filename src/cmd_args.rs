use clap::{Args, Parser, Subcommand};
use semver::Version;

#[derive(Clone, Debug, Parser)]
#[command(about, long_about = None)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub command: Command,

    /// Log more information to the terminal.
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
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
    #[arg(short, long)]
    pub version: Version,

    /// Ask before replacing any existing files.
    #[arg(short, long, default_value_t = true)]
    pub ask: bool,
}

#[derive(Args, Clone, Debug)]
pub struct UninstallArgs {
    /// Doesn't install the config file.
    #[arg(long, default_value_t = false)]
    pub keep_config: bool,
}
