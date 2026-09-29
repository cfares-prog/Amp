use std::{
    fs,
    io,
    path::{Path, PathBuf},
    error::Error
};

pub fn scan_dir(config_path: &str) -> io::Result<()> {
    let path = PathBuf::from(config_path);

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            //TODO: Implement Track record logic
        }
    }

    Ok(())
}
