use crate::ws::WsClient;
use crate::level;
use crate::consts::{YELLOW_COLOR, ESC_COLOR};

pub fn run(clean: bool) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized.".to_string())?;

    if clean {
        println!("{marker}");
    } else {
        println!("Marker of the current level: {YELLOW_COLOR}{marker}{ESC_COLOR}");
    }
    
    Ok(())
}