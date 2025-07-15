mod install;
mod uninstall;

pub use install::*;
pub use uninstall::*;

use core::fmt::Display;
use std::io::Write;

/// The name of seaside's configuration file.
pub const CONFIG_NAME: &str = "Seaside.toml";

/// Asks a yes/no question to the user and returns their answer.
pub fn ask<S>(message: S) -> std::io::Result<bool>
where
    S: Display,
{
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
