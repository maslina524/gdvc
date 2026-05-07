use std::fs;

use crate::terminal::print_by_line;

use crate::ws::WsClient;
use crate::level;
use crate::files::get_level_path;

pub fn run(oneline: bool) -> Result<(), String> {
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
    print_by_line(&lines)?;

    let _ = ws.disconnect();

    Ok(())
}