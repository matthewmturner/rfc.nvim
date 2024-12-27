use std::path::PathBuf;

#[cfg(unix)]
pub fn home_dir() -> Option<PathBuf> {
    // On Unix-like systems, $HOME should be set:
    std::env::var_os("HOME").map(Into::into)
}

#[cfg(windows)]
pub fn home_dir() -> Option<PathBuf> {
    // On Windows, check %USERPROFILE% first; if not set, fall back to %HOMEDRIVE% and %HOMEPATH%.
    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        return Some(user_profile.into());
    }
    let drive = std::env::var_os("HOMEDRIVE");
    let path = std::env::var_os("HOMEPATH");
    match (drive, path) {
        (Some(d), Some(p)) => {
            let mut home = PathBuf::new();
            home.push(d);
            home.push(p);
            Some(home)
        }
        _ => None,
    }
}
