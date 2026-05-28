use std::collections::HashSet;
use std::fs;

use crate::files::get_level_path;
use crate::level::{self, decode_string, get_marker};
use crate::actions::commit::read_commit;

use crate::ws::WsClient;
use crate::consts::{BOLD, ESC};

pub fn run() -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = match get_marker(&string) {
        Some(m) => m,
        None => return Err("The level is not initialized".to_string())
    };

    let head_hash = fs::read_to_string(get_level_path(marker).join("HEAD"))
        .map_err(|e| format!("Failed to read the HEAD file: {e}"))?;
    let path = get_level_path(marker).join("commits").join(&head_hash);

    let head_string = read_commit(&path)?.string;
    let decoded_string = decode_string(&head_string)?;

    let old_sep = decoded_string.find(";").unwrap();
    let old_string = decoded_string[old_sep..].to_string();

    let new_sep = string.find(";").unwrap();
    let new_string = string[new_sep..].to_string();

    let (added, removed) = compare_strings(&new_string, &old_string)?;

    let cur_hash = level::get_string_hash(&string);

    println!(
        "{}index {}..{}{}",
        BOLD, &head_hash[..7], &cur_hash[..7], ESC
    );
    println!("{} insertions(+), {} deletions(-)", added, removed);

    let _ = ws.disconnect();

    Ok(())
}

fn compare_strings(new_string: &String, old_string: &String) -> Result<(usize, usize), String> {
    // NEW STRING
    let new_objects: Vec<&str> = new_string
        .split(";")
        .filter(|s| !s.is_empty()) 
        .collect();

    // OLD STRING
    let old_objects: Vec<&str> = old_string
        .split(";")
        .filter(|s| !s.is_empty())
        .collect();

    let old_ids: HashSet<&str> = old_objects.iter().copied().collect();
    let new_ids: HashSet<&str> = new_objects.iter().copied().collect();
    
    let added = new_ids.difference(&old_ids).count();
    let removed = old_ids.difference(&new_ids).count();
    
    Ok((added, removed))
}