use std::fs;
use std::path::PathBuf;

use plist::{self, Value};

use crate::ws::WsClient;
use crate::level;
use crate::consts::{YELLOW, ESC, GD_PLIST_TAGS_FORMAT};

pub fn run(clean: bool, marker: Option<u32>, gmd: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;

    if let Some(str_path) = gmd {
        let path = PathBuf::from(str_path);
        if !path.is_file() {
            return Err("The gmd path is not a file".into())
        }

        let mut data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read the file: {e}"))?;

        data = data.trim_start().to_string();
        // if let Some(pos) = data.find("<plist") {
        //     data = data[pos..].to_string();
        // }

        for [f, t] in GD_PLIST_TAGS_FORMAT {
            data = data
                .replace(format!("<{f}>").as_str(), format!("<{t}>").as_str())
                .replace(
                    format!("</{f}>").as_str(),
                    format!("</{t}>").as_str()
                )
                .replace(
                    format!("<{f} />").as_str(),
                    format!("<{t} />").as_str()
                );
        }

        let plist: Value = plist::from_bytes(data.as_bytes())
            .map_err(|e| format!("Failed to parse xml: {e}"))?;
        let k4 = plist
            .as_dictionary()
            .ok_or("Gmd is not a dict")?
            .get("k4")
            .ok_or("The k4 key does not exist")?
            .as_string()
            .unwrap()
            .to_string();
        let decoded_string = level::decode_string(&k4)?;
        let new_marker = level::get_marker(&decoded_string)
            .ok_or("There is no marker in the gmd file")?;

        let old_string = ws.get_level_string()?;
        let new_string = level::set_marker(&old_string, new_marker);
        ws.replace_level_string(&new_string)?;

        return Ok(())
    }

    if let Some(m) = marker {
        if m == 0 {
            return Err("Cannot set 0 as the level marker".into())
        }
        let new_string = level::set_marker(&string, m);
        ws.replace_level_string(&new_string)
            .map_err(|e| format!("Failed to update level: {e}"))?;
        
        return Ok(())
    }

    let marker = level::get_marker(&string).ok_or("The level is not initialized".to_string())?;

    if clean {
        println!("{marker}");
    } else {
        println!("Marker of the current level: {YELLOW}{marker}{ESC}");
    }
    
    Ok(())
}