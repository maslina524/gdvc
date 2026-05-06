use crate::ws::WsClient;
use crate::level;
use crate::consts::{YELLOW_COLOR, ESC_COLOR};

pub fn run(clean: bool, marker: Option<u32>) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;

    if let Some(m) = marker {
        if m == 0 {
            return Err("Cannot set 0 as the level marker".to_string())
        }
        let new_string = level::set_marker(&string, m);
        ws.replace_level_string(&new_string)
            .map_err(|e| format!("Failed to update level: {e}"))?;
        
        return Ok(())
    }

    let marker = level::get_marker(&string).ok_or("The level is not initialized.".to_string())?;

    if clean {
        println!("{marker}");
    } else {
        println!("Marker of the current level: {YELLOW_COLOR}{marker}{ESC_COLOR}");
    }
    
    Ok(())
}