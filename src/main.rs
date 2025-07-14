mod cmd_args;
mod common;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

use clap::Parser;
use cmd_args::{CmdArgs, Command};
#[cfg(target_os = "linux")]
use linux::{install, uninstall};
#[cfg(target_os = "windows")]
use windows::{install, uninstall};

fn main() -> std::io::Result<()> {
    let cmd_args = CmdArgs::parse();
    if cmd_args.verbose {
        log::set_max_level(log::LevelFilter::max());
    }
    match cmd_args.command {
        Command::Install(args) => install(args),
        Command::Uninstall(args) => uninstall(args),
    }
}
