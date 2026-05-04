use std::time::SystemTime;
use std::fs::{File, self};
use std::io::{Write, stdin, stdout};

use sha2::{Sha256, Digest};
use hex;

use crate::ws::WsClient;
use crate::level::{self, get_marker, set_marker};
use crate::files;
use crate::consts::{YELLOW_COLOR, ESC_COLOR};

pub fn help() {
    println!("usage: gdvc <command> [<args>]");

    println!("\nstart a working area");
    println!("    init        Initialize your level for gdvc");
    println!("    destroy     Remove all gdvc tracking");

    println!("\nwork on the current");
    println!("    commit      Record changes to the level");
}

pub fn init() -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let mut string = ws.get_level_string()?;
    let marker = level::get_marker(&string);

    if let Some(_) = marker {
        println!("Gdvc is already initialized at this level.");
        return Ok(())
    }

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    
    let new_string = set_marker(&mut string, timestamp);
    let _ = ws.replace_level_string(&new_string);

    let _ = ws.disconnect();

    Ok(())
}

pub fn destroy(force: bool, hard: bool) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let mut string = ws.get_level_string()?;

    let marker = match get_marker(&string) {
        Some(m) => m,
        None => return Err("The level is not initialized.".to_string())
    };

    if !force {
        println!("This action will remove all gdvc tracking from your level.");
        println!("This operation is irreversible.\n");

        if hard {
            println!("[!] With --hard, this will also permanently delete the entire");
            println!("    level directory for this level from your disk.");
            println!("    All commits will be lost forever.\n");
        }

        println!("Type {}YES{} to confirm:", YELLOW_COLOR, ESC_COLOR);
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).map_err(|_| "Did not enter a correct string")?;
        input = input.trim().to_string();

        if input != "YES" {
            return Ok(());
        }
    }

    if hard {
        let path = files::get_level_path(marker);
        let _ = fs::remove_dir_all(&path);
    }
    
    let new_string = set_marker(&mut string, 0);
    let _ = ws.replace_level_string(&new_string);

    let _ = ws.disconnect();

    Ok(())
}

pub fn commit(message: &String) -> Result<(), String> {
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
    let full_hash = hex::encode(hash);
    let hex_hash = &full_hash[..7];

    let commit_path = files::get_level_path(marker).join("commits").join(hex_hash);
    let mut file = File::create(commit_path)
        .map_err(|e| format!("Failed to create commit file: {}", e))?;

    let _ = file.write_all(&file_data.as_bytes());

    Ok(())
}