use std::fs;

use crate::actions::commit::{read_commit, sort_commits};
use crate::terminal::print_by_line;

use crate::ws::WsClient;
use crate::level;
use crate::files::get_level_path;

pub fn run(oneline: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized".to_string())?;

    let path = get_level_path(marker).join("commits");
    let files = fs::read_dir(path)
        .map_err(|e| format!("Failed to get commits: {e}"))?;

    let mut commits = vec![];
    for file in files {
        let file = file.unwrap().path();
        let cur_commit = read_commit(&file)?;
        commits.push(cur_commit);
    }

    if commits.is_empty() {
        return Ok(())
    }
    
    sort_commits(&mut commits);
    commits.reverse();

    let mut lines = vec![];
    let head_path = get_level_path(marker).join("HEAD");
    let head_hash = fs::read_to_string(&head_path)
        .map_err(|e| format!("Failed to read the HEAD file: {e}"))?;
    for commit in commits {
        let is_head = commit.hash == head_hash;
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
    }
    print_by_line(&lines)?;

    let _ = ws.disconnect();

    Ok(())
}