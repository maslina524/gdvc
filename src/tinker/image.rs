use std::path::PathBuf;

use base64::prelude::*;

use crate::{files::get_tinker_path, object::GameObject};

/// Reads the level string and returns a vector of image paths used in the level as Reference Images and push new string to buf
pub fn get_reference_images_from_string(string: &str, marker: u32, buf: &mut String) -> Vec<String> {
    let mut ret = Vec::new();
    buf.clear();

    let sep_idx = match string.find(';') {
        Some(i) => i,
        None => return ret,
    };
    let header = &string[..sep_idx];
    let objects_str = &string[sep_idx + 1..];
    let objects = objects_str.split(';').collect::<Vec<&str>>();

    let mut new_objects = Vec::with_capacity(objects.len());

    for obj_str in objects {
        let mut obj = match GameObject::from(obj_str) {
            Some(o) => o,
            None => {
                new_objects.push(obj_str.to_string());
                continue;
            }
        };

        if let Some(value) = obj.props.get(&31) {
            let text = match BASE64_URL_SAFE.decode(value) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(_) => {
                        new_objects.push(obj_str.to_string());
                        continue;
                    }
                },
                Err(_) => {
                    new_objects.push(obj_str.to_string());
                    continue;
                }
            };

            if !text.starts_with("image:") {
                new_objects.push(obj_str.to_string());
                continue;
            }

            let colon_pos = match text.find(':') {
                Some(p) => p,
                None => {
                    new_objects.push(obj_str.to_string());
                    continue;
                }
            };
            let b64_path = &text[colon_pos + 1..];

            let path_str = match BASE64_URL_SAFE.decode(b64_path) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(_) => {
                        new_objects.push(obj_str.to_string());
                        continue;
                    }
                },
                Err(_) => {
                    new_objects.push(obj_str.to_string());
                    continue;
                }
            };

            ret.push(path_str.clone());

            let path = PathBuf::from(&path_str);
            let file_name = match path.file_name().and_then(|f| f.to_str()) {
                Some(name) => name.to_owned(),
                None => {
                    new_objects.push(obj_str.to_string());
                    continue;
                }
            };
            let file_path = get_tinker_path(marker).join(file_name);
            let file_path_str = file_path.to_str().unwrap();

            let mut encoded_name = String::new();
            BASE64_URL_SAFE.encode_string(file_path_str, &mut encoded_name);

            let new_text = format!("image:{}", encoded_name);
            let mut encoded_new_text = String::new();
            BASE64_URL_SAFE.encode_string(&new_text, &mut encoded_new_text);
            
            obj.props.insert(31, encoded_new_text);

            new_objects.push(obj.to_string());
        } else {
            new_objects.push(obj_str.to_string());
        }
    }

    let new_level_string = if new_objects.is_empty() {
        header.to_string()
    } else {
        format!("{};{}", header, new_objects.join(";"))
    };

    buf.push_str(&new_level_string);
    ret
}

#[cfg(test)]
mod tests {
    use crate::{tinker::image::get_reference_images_from_string, ws::WsClient};

    #[test]
    fn get_ref_images() {
        let mut ws = WsClient::connect().unwrap();
        let string = ws.get_level_string().unwrap();
        
        let content = get_reference_images_from_string(&string, 0, &mut String::new());
        dbg!(content);
    }
}