use std::path::Path;

/// Uninstalls seaside's global configuration file.
pub fn uninstall_config<P>(parent_directory: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    eprintln!("\x1b[38;5;248muninstalling config...\x1b[0m");

    remove_dir_all(parent_directory, "config")?;

    eprintln!("\x1b[38;5;248msuccessfully uninstalled config\x1b[0m");
    Ok(())
}

/// Tries to remove the given directory and all of its descendants.
///
/// This will not throw an error if `directory` does not exist.
pub fn remove_dir_all<P>(directory: P, repr: &'static str) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    match std::fs::remove_dir_all(directory) {
        Ok(()) => {
            eprintln!("\x1b[38;5;248msuccessfully removed {repr}\x1b[0m");
            Ok(())
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("\x1b[38;5;248m{repr} was not present\x1b[0m");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
