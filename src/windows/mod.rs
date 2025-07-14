mod install;
mod path;
mod uninstall;

pub use install::install;
pub use uninstall::uninstall;

const BINARY_RELEASE_NAME_MSVC: &str = "x86_64-pc-windows-msvc-seaside.exe";
const BINARY_RELEASE_NAME_GNU: &str = "x86_64-pc-windows-gnu-seaside.exe";
const SEASIDE_PROGRAM_DATA: &str = r"C:\ProgramData\seaside";
const BINARY_PATH: &str = r"C:\ProgramData\seaside\seaside.exe";
