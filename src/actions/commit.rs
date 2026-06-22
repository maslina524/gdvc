use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::fs::File;
use std::io::Write;

use chrono::DateTime;
use chrono::FixedOffset;

use crate::consts::BLUE;
use crate::consts::ESC;
use crate::consts::YELLOW;
use crate::files::get_tinker_path;
use crate::tinker;
use crate::ws::WsClient;
use crate::level;
use crate::files;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub timestamp: u32,
    pub message: String,
    pub string: String
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

pub fn read_commit(path: &PathBuf) -> Result<Commit, String> {
    let data = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read the commit file: {e}"))?;

    let parts: Vec<&str> = data.split("\n\n").collect();
    let (meta, string) = (parts[0], parts[1].to_string());

    let lines_meta: Vec<&str> = meta.lines().collect();

    let hash = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("no hash")
        .to_string();

    let timestamp = lines_meta
        .get(0)
        .and_then(|t| t.parse::<u32>().ok())
        .unwrap_or(0);

    let message = lines_meta
        .get(1)
        .cloned()
        .map(|s| s.to_string())
        .unwrap_or(String::new());

    Ok(
        Commit {
            hash: hash,
            timestamp: timestamp,
            message: message,
            string: string
        }
    )
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

pub fn commit(message: &String, amend: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WsClient::connect()?;

    let string = ws.get_level_string()?;
    let marker = level::get_marker(&string).ok_or("The level is not initialized".to_string())?;

    if amend {
        let head_hash = fs::read_to_string(files::get_level_path(marker).join("HEAD"))
            .map_err(|e| format!("Failed to read the HEAD file: {e}"))?;

        let path = files::get_level_path(marker).join("commits").join(head_hash);
        let commit = read_commit(&path)?;

        let mut file = File::create(path)
            .map_err(|e| format!("Failed to create commit file: {}", e))?;

        let file_data= vec![
            &commit.timestamp.to_string(),
            message,
            "",
            &commit.string
        ].join("\n");

        let _ = file.write_all(&file_data.as_bytes());
        return Ok(())
    }

    files::create_level_folder(marker)?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let mut new_string = String::new();
    let img_paths = tinker::image::get_reference_images_from_string(&string, marker, &mut new_string);
    for img in &img_paths {
        let name = Path::new(&img).file_name().unwrap();
        let path = get_tinker_path(marker).join(name);
        fs::copy(img, path)?;
    }
    let string = new_string;

    if !img_paths.is_empty() {
        ws.replace_level_string(&string)?;
    }

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

#[cfg(test)]
mod tests {
    use crate::{actions::commit::read_commit, files::get_level_path};

    #[test]
    fn read_commit_test() {
        let commit_hash = "2f789befcea09a9304708e87f246277c99e3d19d7c9072d5afa1e892ac83a5b7";
        let path = get_level_path(1778784712).join("commits").join(commit_hash);

        let commit_data = read_commit(&path);
        println!("{commit_data:#?}")
    }
}