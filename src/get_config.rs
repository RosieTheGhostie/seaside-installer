use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub fn dir() -> std::io::Result<PathBuf> {
    let mut path = PathBuf::from("/home");
    path.push(crate::user::user());
    path.push(".config/seaside");
    Ok(path)
}

#[cfg(not(target_os = "linux"))]
pub fn dir() -> std::io::Result<PathBuf> {
    let project_dirs = match directories::ProjectDirs::from("", "", "seaside") {
        Some(project_dirs) => project_dirs,
        None => return Err(std::io::Error::from(std::io::ErrorKind::NotFound)),
    };
    Ok(project_dirs.config_dir().to_path_buf())
}

pub fn path() -> std::io::Result<PathBuf> {
    let mut dir = dir()?;
    dir.push(crate::consts::CONFIG_NAME);
    Ok(dir)
}
