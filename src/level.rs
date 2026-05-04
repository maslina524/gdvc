use std::io::Write;

use libflate::gzip::{Encoder, Decoder};
use base64::prelude::*;

use crate::consts::SECRET_KEY;

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
                let shifted = timestamp as i64 - (i32::MIN as i64);
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

#[cfg(test)]
mod tests {
    use crate::level::encode_string;

    #[test]
    fn encode_string_test() {
        let string = String::from("1,914,2,15,3,45;");
        let encoded = encode_string(&string).unwrap();
        
        assert_eq!(encoded, "H4sIAPG--GkAAwXAsREAAAjCwIXSoFB47r8XL05mUFicL48ckc0QAAAA")
    }
}