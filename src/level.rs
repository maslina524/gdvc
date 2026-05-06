use std::io::{self, Read, Write, BufRead, BufReader};
use std::fs::File;
use std::path::PathBuf;

use chrono::{DateTime, FixedOffset};
use libflate::gzip::{Encoder, Decoder};
use base64::prelude::*;

use crate::consts::{SECRET_KEY, YELLOW_COLOR, BLUE_COLOR, ESC_COLOR};

pub fn get_marker(string: &str) -> Option<u32> {
    let semicolon_pos = string.find(';')?;
    let before = &string[..semicolon_pos];
    
    let pairs: Vec<&str> = before.split(',').collect();
    for chunk in pairs.chunks(2) {
        if chunk.len() == 2 && chunk[0] == SECRET_KEY {
            let result = chunk[1].parse::<i32>();
            if let Ok(marker) = result {
                if marker != 0 {
                    return Some((marker as i64 + (i32::MIN as i64)) as u32)
                }
            }
            return None
        }
    }
    None
}

pub fn set_marker(string: &String, timestamp: u32) -> String {
    let split_i = string.find(';').unwrap();
    let object_string = &string[split_i..];

    let parts_before = &string[..split_i];
    let tokens: Vec<&str> = parts_before.split(',').collect();
    let mut new_parts = Vec::new();

    for chunk in tokens.chunks(2) {
        if chunk.len() == 2 {
            let key = chunk[0];
            let value = if key == SECRET_KEY {
                let shifted = if timestamp != 0 {
                    timestamp as i64 - (i32::MIN as i64)
                } else {
                    0
                };
                (shifted as i32).to_string()
            } else {
                chunk[1].to_string()
            };
            new_parts.push(format!("{},{}", key, value));
        }
    }

    let mut ret = new_parts.join(",");
    ret.push_str(object_string);
    ret
}

/// https://boomlings.dev/topics/levelstring_encoding_decoding#encoding
pub fn encode_string(string: &String) -> Result<String, String> {
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    encoder.write_all(string.as_ref()).unwrap();

    let gzipped = encoder
        .finish()
        .into_result()
        .map_err(|_| "Failed to compress the level using gzip.")?;

    let base64_encoded = BASE64_URL_SAFE.encode(gzipped);

    Ok(base64_encoded)
}

/// https://boomlings.dev/topics/levelstring_encoding_decoding#decoding
pub fn decode_string(string: &String) -> Result<String, String> {
    let base64_decoded = BASE64_URL_SAFE.decode(string).unwrap();

    let mut decoder = Decoder::new(&base64_decoded[..]).unwrap();
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|_| "Failed to compress the level using gzip.")?; 

    let decompressed_string = String::from_utf8(decompressed).unwrap();
    Ok(decompressed_string)
}

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
            YELLOW_COLOR, self.hash, ESC_COLOR
        ));

        if is_head {
            ret.push_str(&format!(
                "{} <- {}HEAD{}",
                YELLOW_COLOR, BLUE_COLOR, ESC_COLOR
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
                YELLOW_COLOR, &self.hash[..7], BLUE_COLOR, YELLOW_COLOR, ESC_COLOR, self.message
            );
        } else {
            ret = format!(
                "{}{}{} {}",
                YELLOW_COLOR, &self.hash[..7], ESC_COLOR, self.message
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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{files::get_level_path, level::{decode_string, encode_string, read_commit_meta}};

    #[test]
    fn encode_string_test() {
        let string = String::from("1,914,2,15,3,45;");
        let encoded = encode_string(&string);
        
        assert!(encoded.is_ok())
    }

    #[test]
    fn decode_string_test() {
        let string = String::from("1,914,2,15,3,45;");
        let encoded = encode_string(&string).unwrap();
        let decoded = decode_string(&encoded).unwrap();
        
        assert_eq!(decoded, string)
    }

    #[test]
    fn read_commit_meta_test() {
        let path = get_level_path(1777940517).join("commits");
        let files = fs::read_dir(path).unwrap();
        for file in files {
            println!("{:?}", read_commit_meta(file.unwrap().path()).unwrap());
        }
    }
}