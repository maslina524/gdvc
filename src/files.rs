use std::path::PathBuf;
use std::fs;

use dirs::data_local_dir;

pub fn get_gdvc_path() -> PathBuf {
    data_local_dir().unwrap().join(".gdvc")
}

pub fn get_level_path(marker: u32) -> PathBuf {
    get_gdvc_path().join(marker.to_string())
}

pub fn create_level_folder(marker: u32) -> Result<(), String> {
    let path = get_level_path(marker);

    fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create base level directory: {}", e))?;

    fs::create_dir_all(&path.join("commits"))
        .map_err(|e| format!("Failed to create commits directory: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::files::get_level_path;

    #[test]
    fn get_level_path_test() {
        if cfg!(target_os = "windows") {
            let path = get_level_path(1u32).display().to_string();
            println!("{path}")
        } else {
            panic!("Run on Windows!")
        }
    }
}