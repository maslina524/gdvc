use std::fs;

use crate::actions::commit::{Commit, read_commit_meta, read_commit_string, sort_commits};
use crate::ws::WsClient;
use crate::level::{self, decode_string};
use crate::files::{self, get_level_path};

pub fn run(target: String, hard: bool) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized.".to_string())?;

    let path = get_level_path(marker).join("commits");
    let files = fs::read_dir(path)
        .map_err(|e| format!("Failed to get commits: {e}"))?;

    let mut commits = vec![];
    for file in files {
        let file = file.unwrap().path();
        let cur_commit = read_commit_meta(file)
            .map_err(|e| format!("Failed to get commit meta: {e}"))?;
        commits.push(cur_commit);
    }

    let head_hash = fs::read_to_string(get_level_path(marker).join("HEAD"))
        .map_err(|e| format!("Failed to read the HEAD file: {e}"))?;
    let target_commit = get_target_commit(&mut commits, &target, &head_hash)?;
    files::create_head_file(marker, &target_commit.hash)?;

    if hard {
        let path = get_level_path(marker).join("commits").join(&target_commit.hash);
        let encoded_string = read_commit_string(path)
            .map_err(|e| format!("Io error: {e}"))?;

        let level_string = decode_string(&encoded_string)?;
        ws.replace_level_string(&level_string)
            .map_err(|e| format!("Failed to update level: {e}"))?;
    }

    let _ = ws.disconnect();
    
    Ok(())
}

fn get_target_commit<'a>(mut commits: &'a mut [Commit], target: &String, head_hash: &String) -> Result<&'a Commit, String> {
    sort_commits(&mut commits);
    commits.reverse();

    // HEAD~N && HEAD
    if target.starts_with("HEAD") {
        let head_i = commits.iter()
            .position(|c| &c.hash == head_hash)  // полное совпадение
            .ok_or("HEAD commit not found")?;

        let i = match target.find('~') {
            Some(index) => {
                target[index + 1..].parse::<usize>()
                    .map_err(|_| "Incorrect index.")?
            },
            None => 0
        };
        if head_i + i >= commits.len() {
            return Err("Commit index out of range".to_string());
        }
        return Ok(&commits[head_i + i])
    }
    
    // timestamp
    if let Ok(timestamp) = target.parse::<u32>() {
        for (idx, commit) in commits.iter().enumerate() {
            if commit.timestamp == timestamp {
                return Ok(&commits[idx])
            }
        }
        return Err("No commit with this timestamp was found.".to_string())
    }

    // hash
    if is_hex_string(&target) {
        for (idx, commit) in commits.iter().enumerate() {
            let hash = &commit.hash[..target.len() as usize];
            if hash == target {
                return Ok(&commits[idx])
            }
        }
        return Err("No commit with this hash was found.".to_string())
    }

    Err("Incorrect options for specifying commits.".to_string())
}

fn is_hex_string(s: &String) -> bool {
    s.len() >= 4 && s.chars().all(|c| c.is_ascii_hexdigit())
}