mod cmd_args;
mod consts;
mod get_config;
mod install;
mod uninstall;
#[cfg(target_os = "linux")]
mod user;
#[cfg(target_os = "windows")]
mod windows_path;

use clap::Parser;
use cmd_args::{CmdArgs, Command};
use install::install;
use std::process::ExitCode;
use uninstall::uninstall;

fn main() -> ExitCode {
    let cmd_args = CmdArgs::parse();
    #[cfg(target_os = "linux")]
    if let Some(user) = user::get_sudo_user() {
        // this is guaranteed to be the first and only time we set `USER`
        let _ = user::USER.set(user);
    } else {
        eprintln!("\x1b[31m[ERROR] must run via sudo\x1b[0m");
        return ExitCode::FAILURE;
    }
    let result = match cmd_args.command {
        Command::Install(args) => install(args),
        Command::Uninstall(args) => uninstall(args),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("\x1b[31m[ERROR] {err}\x1b[0m");
            ExitCode::FAILURE
        }
    }
}

/// Asks a yes/no question to the user and returns their answer.
pub fn ask<S>(message: S) -> std::io::Result<bool>
where
    S: core::fmt::Display,
{
    use std::io::Write;

    #[cfg(not(target_os = "windows"))]
    const NEWLINE: &str = "\n";
    #[cfg(target_os = "windows")]
    const NEWLINE: &str = "\r\n";

    loop {
        print!("{message} (y/n) > ");
        std::io::stdout().flush()?;
        let mut temp = String::new();
        std::io::stdin().read_line(&mut temp)?;
        temp.make_ascii_lowercase();
        match temp.strip_suffix(NEWLINE).unwrap_or(&temp) {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => eprintln!("invalid response. please try again"),
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
