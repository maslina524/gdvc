use std::time::SystemTime;
use std::fs::File;
use std::io::Write;

use sha2::{Sha256, Digest};
use hex;

use crate::ws::WsClient;
use crate::level;
use crate::files;

pub fn run(message: &String) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized.".to_string())?;

    files::create_level_folder(marker)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let encoded_string = level::encode_string(&string)?;

    let file_data = vec![
        timestamp.to_string().as_str(),
        message,
        "",
        &encoded_string
    ].join("\n");

    let hash = Sha256::digest(&file_data);
    let hex_hash = hex::encode(hash);

    let commit_path = files::get_level_path(marker).join("commits").join(&hex_hash);
    let mut file = File::create(commit_path)
        .map_err(|e| format!("Failed to create commit file: {}", e))?;

    let _ = file.write_all(&file_data.as_bytes());

    // HEAD file
    files::create_head_file(marker, &hex_hash)?;
    
    Ok(())
}