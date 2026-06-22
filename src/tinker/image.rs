use base64::prelude::*;

use crate::level::parse_obj;

/// Reads the level string and returns a vector of image paths used in the level as Reference Images
pub fn get_reference_images_from_string(string: &str) -> Vec<String> {
    let mut ret = Vec::new();

    let sep_idx = match string.find(";") {
        Some(i) => i,
        None => return ret
    };
    let objects_str = &string[sep_idx + 1..];
    let objects = objects_str.split(";").collect::<Vec<&str>>();

    for obj in objects {
        let props = match parse_obj(obj) {
            Some(o) => o,
            None => continue
        };

        // All text objects
        if let Some(value) = props.get(&31) {
            let text = match BASE64_URL_SAFE.decode(value) {
                Ok(c) => String::from_utf8(c).unwrap(),
                Err(_) => continue
            };

            if !text.starts_with("image:") { continue; }

            let sep_idx = text.find(":").unwrap();
            let b64_path = &text[sep_idx + 1..];

            let path = match BASE64_URL_SAFE.decode(b64_path) {
                Ok(c) => String::from_utf8(c).unwrap(),
                Err(_) => continue
            };

            ret.push(path);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::{tinker::image::get_reference_images_from_string, ws::WsClient};

    #[test]
    fn get_ref_images() {
        let mut ws = WsClient::connect().unwrap();
        let string = ws.get_level_string().unwrap();

        let content = get_reference_images_from_string(&string);
        dbg!(content);
    }
}