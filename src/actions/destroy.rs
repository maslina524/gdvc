use std::fs;
use std::io::{Write, stdin, stdout};

use crate::ws::WsClient;
use crate::level::{get_marker, set_marker};
use crate::files;
use crate::consts::{ESC, YELLOW};

pub fn run(force: bool, hard: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WsClient::connect()?;

    let mut string = ws.get_level_string()?;

    let marker = match get_marker(&string) {
        Some(m) => m,
        None => return Err("The level is not initialized".into())
    };

    if !force {
        println!("This action will remove all gdvc tracking from your level");
        println!("This operation is irreversible\n");

        if hard {
            println!("[!] With --hard, this will also permanently delete the entire");
            println!("    level directory for this level from your disk");
            println!("    All commits will be lost forever\n");
        }

        println!("Type {}YES{} to confirm:", YELLOW, ESC);
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