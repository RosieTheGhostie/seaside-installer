#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "windows")]
pub use windows::*;

/// The name of seaside's configuration file.
pub const CONFIG_NAME: &str = "Seaside.toml";

#[cfg(target_os = "linux")]
mod linux {
    pub const BINARY_RELEASE_NAME: &str = "x86_64-unknown-linux-gnu-seaside";

    pub const BINARY_PATH: &str = "/usr/local/bin/seaside";
}

#[cfg(target_os = "windows")]
mod windows {
    pub const BINARY_RELEASE_NAME_MSVC: &str = "x86_64-pc-windows-msvc-seaside.exe";
    pub const BINARY_RELEASE_NAME_GNU: &str = "x86_64-pc-windows-gnu-seaside.exe";

    pub const BINARY_DIRECTORY: &str = r"C:\ProgramData\seaside";
    pub const BINARY_PATH: &str = r"C:\ProgramData\seaside\seaside.exe";
}
