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
use std::process::ExitCode;
#[cfg(target_os = "windows")]
use windows::{install, uninstall};

fn main() -> ExitCode {
    let cmd_args = CmdArgs::parse();
    let result = match cmd_args.command {
        Command::Install(args) => install(args),
        Command::Uninstall(args) => uninstall(args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\x1b[31m[ERROR] {err}\x1b[0m");
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("you may need to run this as root/admin");
            }
            ExitCode::FAILURE
        }
    }
}

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        eprintln!();
    };
    ($($arg:tt)*) => {{
        #![cfg(debug_assertions)]
        eprint!("\x1b[35m[DEBUG] ");
        eprint!($($arg)*);
        eprintln!("\x1b[0m");
    }};
}
