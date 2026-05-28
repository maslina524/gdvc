use std::time::SystemTime;

use crate::files;
use crate::ws::WsClient;
use crate::level::{self, set_marker};

pub fn run(quiet: bool) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let mut string = ws.get_level_string()?;
    let marker = level::get_marker(&string);

    if let Some(_) = marker {
        eprintln!("Gdvc is already initialized at this level.");
        return Ok(())
    }

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    
    let new_string = set_marker(&mut string, timestamp);
    let _ = ws.replace_level_string(&new_string);

    if quiet {
        println!("Initialized empty Gdvc repository in {}.", files::get_level_path(timestamp).display());
    }

    let _ = ws.disconnect();

    Ok(())
}