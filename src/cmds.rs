use std::time::SystemTime;

use crate::ws::WsClient;
use crate::level::{self, set_marker};

pub fn help() {
    println!("usage: gdvc <command> [<args>]\n");

    println!("start a working area");
    println!("    init        Initialize your level for Gdvc")
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