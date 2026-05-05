use std::time::SystemTime;
use std::fs::{File, self};
use crossterm::{cursor, ExecutableCommand, terminal};
use std::io::{Read, Write, stdin, stdout};

use sha2::{Sha256, Digest};
use hex;

use crate::ws::WsClient;
use crate::level::{self, get_marker, set_marker};
use crate::files::{self, get_level_path};
use crate::consts::{ESC_COLOR, YELLOW_COLOR};

pub fn help() {
    println!("usage: gdvc [-v | --version] [-p | --path]");
    println!("            <command> [<args>]");

    println!("\nstart a working area");
    println!("    init        Initialize your level for Gdvc");
    println!("    destroy     Remove all Gdvc tracking");

    println!("\nexamine the history and state");
    println!("    log         Show commit logs");

    println!("\nwork on the current");
    println!("    commit      Record the changes");
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

pub fn log(oneline: bool) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized.".to_string())?;

    let path = get_level_path(marker).join("commits");
    let files = fs::read_dir(path)
        .map_err(|e| format!("Failed to get commits: {e}"))?;

    let mut commits = vec![];
    for file in files {
        let file = file.unwrap().path();
        let cur_commit = level::read_commit_meta(file)
            .map_err(|e| format!("Failed to get commit meta: {e}"))?;
        commits.push(cur_commit);
    }

    if commits.is_empty() {
        return Ok(())
    }
    
    level::sort_commits(&mut commits);
    commits.reverse();

    let mut lines = vec![];
    let mut is_head = true;
    for commit in commits {
        if oneline {
            lines.push(commit.format_oneline(is_head));
        } else {
            let mut commit_lines: Vec<String> = commit
                .format_multiline(is_head)
                .split('\n')
                .map(|s| s.to_string())
                .collect();
            lines.append(&mut commit_lines);
        }
        is_head = false;
    }

    if lines.len() <= 14 {
        for l in lines {
            println!("{l}");
        }
    } else {
        let mut i = 0;
        println!("{}", lines[i]);
        while i != lines.len() - 1 {
            print!(": ");

            let mut stdout = stdout();
    
            let _ = terminal::enable_raw_mode();
            
            let _ = stdout.flush();
            
            let mut buffer = [0; 1];
            let _ = std::io::stdin().read_exact(&mut buffer);
            let symb = buffer[0] as char;
            
            let _ = stdout.execute(cursor::MoveLeft(1));
            let _ = stdout.execute(terminal::Clear(terminal::ClearType::UntilNewLine));

            let _ = stdout.flush();
            
            let _ = terminal::disable_raw_mode();

            if symb == 'q' {
                let _ = terminal::disable_raw_mode();
                return Ok(());
            }
            i += 1;
            print!("\r\x1B[2K");
            println!("{}", lines[i]);
        }
    }

    let _ = ws.disconnect();
    let _ = terminal::disable_raw_mode();
    Ok(())
}