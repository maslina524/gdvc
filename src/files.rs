use std::io::Write;
use std::path::PathBuf;
use std::fs::{self, File};

use dirs::data_local_dir;
use serde_json::Value;

pub fn get_mod_settings(mod_id: &str) -> Option<Value> {
    let path = data_local_dir().unwrap()
        .join("GeometryDash").join("geode").join("mods").join(mod_id).join("settings.json");

    let data_str = fs::read_to_string(path).ok()?;
    let json = serde_json::from_str(&data_str).ok();
    json
}

pub fn get_gdvc_path() -> PathBuf {
    let dir = data_local_dir().unwrap().join(".gdvc");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn get_tinker_path(marker: u32) -> PathBuf {
    let dir = get_level_path(marker).join("tinker");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn get_level_path(marker: u32) -> PathBuf {
    let dir = get_gdvc_path().join(marker.to_string());
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn create_level_folder(marker: u32) -> Result<(), String> {
    let path = get_level_path(marker);

    fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create base level directory: {}", e))?;

    fs::create_dir_all(&path.join("commits"))
        .map_err(|e| format!("Failed to create commits directory: {}", e))?;

    Ok(())
}

pub fn create_head_file(marker: u32, hash: &String) -> Result<(), String> {
    let head_path = get_level_path(marker).join("HEAD");
    let mut file = File::create(head_path)
        .map_err(|e| format!("Failed to create the HEAD file: {e}"))?;
    let _ = file.write_all(hash.as_bytes());

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