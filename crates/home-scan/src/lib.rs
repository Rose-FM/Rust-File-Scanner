use std::env;
use std::fs;
use std::path::PathBuf;

pub fn scan_home_directory() -> Result<Vec<PathBuf>, std::io::Error> {
    let home_dir = if cfg!(target_os = "windows") {
        env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string())
    } else {
        env::var("HOME").unwrap_or_else(|_| "/home/default".to_string())
    };

    let mut directories = Vec::new();
    for entry in fs::read_dir(home_dir)? {
        let entry = entry?;
        if entry.path().is_dir() {
            directories.push(entry.path());
        }
    }

    Ok(directories)
}