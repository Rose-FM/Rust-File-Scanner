use std::fs;
use std::path::Path;
use std::io;

pub fn scan_directory<P: AsRef<Path>>(path: P) -> (Vec<String>, Vec<String>) {
    let mut dirs = Vec::new();
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path.to_string_lossy().to_string());
                } else if path.is_file() {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    (dirs, files)
}
