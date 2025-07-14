mod cmd_args;
mod common;
#[cfg(target_os = "linux")]
mod linux;
mod logging;
#[cfg(target_os = "windows")]
mod windows;

use clap::Parser;
use cmd_args::{CmdArgs, Command};
#[cfg(target_os = "linux")]
use linux::{install, uninstall};
use std::process::ExitCode;
#[cfg(target_os = "windows")]
use windows::{install, uninstall};

fn main() -> ExitCode {
    let cmd_args = CmdArgs::parse();
    if let Some(log_level) = cmd_args.log {
        unsafe { logging::LOG_LEVEL = log_level };
    }
    let result = match cmd_args.command {
        Command::Install(args) => install(args),
        Command::Uninstall(args) => uninstall(args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            minimal_logging::macros::fatalln!("{err}");
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("you may need to run this as root/admin");
            }
            ExitCode::FAILURE
        }
    }
}
