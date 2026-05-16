use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::SystemTime;
use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::FixedOffset;

use crate::consts::BLUE;
use crate::consts::ESC;
use crate::consts::YELLOW;
use crate::ws::WsClient;
use crate::level;
use crate::files;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub timestamp: u32,
    pub message: String
}

impl Commit {
    pub fn format_multiline(&self, is_head: bool) -> String {
        let mut ret = String::new();

        ret.push_str(&format!(
            "{}commit {}{}",
            YELLOW, self.hash, ESC
        ));

        if is_head {
            ret.push_str(&format!(
                "{} <- {}HEAD{}",
                YELLOW, BLUE, ESC
            ));
        }
        ret.push('\n');

        let dt = DateTime::from_timestamp(self.timestamp as i64, 0)
            .expect("Invalid timestamp")
            .with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap());
        
        let timestamp_str = dt.format("%a %b %e %H:%M:%S %Y %z").to_string();
        ret.push_str(&format!("Date: {timestamp_str}\n"));
        ret.push('\n');
        ret.push_str(&format!("    {}\n", self.message));
        ret.push('\n');

        ret
    }
    
    pub fn format_oneline(&self, is_head: bool) -> String {
        let ret;
        if is_head {
            ret = format!(
                "{}{} <- ({}HEAD{}){} {}",
                YELLOW, &self.hash[..7], BLUE, YELLOW, ESC, self.message
            );
        } else {
            ret = format!(
                "{}{}{} {}",
                YELLOW, &self.hash[..7], ESC, self.message
            );
        }
        ret
    }
}

pub fn read_commit_meta(path: PathBuf) -> io::Result<Commit> {
    // Reading meta-data from file
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    
    // Generate Commit struct
    let hash = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("no hash")
        .to_string();

    let timestamp = lines
        .get(0)
        .and_then(|t| t.parse::<u32>().ok())
        .unwrap_or(0);

    let message = lines
        .get(1)
        .cloned()
        .unwrap_or(String::new());

    Ok(
        Commit {
            hash: hash,
            timestamp: timestamp,
            message: message
        }
    )
}

pub fn read_commit_string(path: PathBuf) -> io::Result<String> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut prev_line_is_empty = false;

    for line in reader.lines() {
        let line = line?;
        if prev_line_is_empty {
            return Ok(line)
        }
        if line.is_empty() {
            prev_line_is_empty = true;
        }
    }

    Ok(String::new())
}

pub fn sort_commits(commits: &mut [Commit]) {
    // INSERTION SORT BY TIMESTAMP
    for i in 1..commits.len() {
        let key = commits[i].clone();
        let mut j = i as i32 - 1;

        while j >= 0 && commits[j as usize].timestamp > key.timestamp {
            commits[(j + 1) as usize] = commits[j as usize].clone();
            j -= 1;
        }
        commits[(j + 1) as usize] = key;
    }
}

pub fn run(message: &String) -> Result<(), String> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized.".to_string())?;

    files::create_level_folder(marker)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let encoded_string = level::encode_string(&string)?;
    
    let hex_hash = level::get_string_hash(&string);

    let commit_path = files::get_level_path(marker).join("commits").join(&hex_hash);
    let mut file = File::create(commit_path)
        .map_err(|e| format!("Failed to create commit file: {}", e))?;

    let file_data = vec![
        timestamp.to_string().as_str(),
        message,
        "",
        &encoded_string
    ].join("\n");

    let _ = file.write_all(&file_data.as_bytes());

    // HEAD file
    files::create_head_file(marker, &hex_hash)?;
    
    Ok(())
}