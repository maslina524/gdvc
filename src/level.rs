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

pub fn set_timestamp(string: &String, timestamp: u32) -> String {
    let split_i = string.find(';').unwrap();
    let object_string = &string[split_i..]; // часть после ';', включая ';'

    let parts_before = &string[..split_i];
    let tokens: Vec<&str> = parts_before.split(',').collect();
    // Ожидаем чётное количество токенов: ключ1, значение1, ключ2, значение2, ...
    let mut new_parts = Vec::new();

    for chunk in tokens.chunks(2) {
        if chunk.len() == 2 {
            let key = chunk[0];
            let value = if key == "kA26" {
                // Вычисляем сдвиг: timestamp -> i32 с помощью i32::MIN
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