pub fn get_timestamp(string: &String) -> Option<u32> {
    let split_i = string.find(';').unwrap();
    
    let binding = string[..split_i]
        .split(",")
        .collect::<Vec<&str>>();
    
    let props = binding.chunks(2).collect::<Vec<&[&str]>>();

    for prop in props {
        if prop[0] != "kA26" {
            continue;
        }
        
        let result = prop[1].parse::<i32>();
        if let Ok(n) = result {
            return Some((n as i32 + i32::MIN) as u32)
        } else {
            return None
        }
    }

    None
}