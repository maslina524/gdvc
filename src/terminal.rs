use std::io::{Write, stdin, stdout};

use crate::consts::{ESC, INVERT, LINES_THRESHOLD};

pub fn print_by_line_str(text: String) -> Result<(), String> {
    let lines: Vec<String> = text
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    print_by_line(lines)
}

pub fn print_by_line(lines: Vec<String>) -> Result<(), String> {
    if lines.len() <= LINES_THRESHOLD {
        println!("{}", lines.join("\n"));
        return Ok(())
    }

    for i in 0..LINES_THRESHOLD {
        println!("{}", lines[i])
    }

    let mut i = LINES_THRESHOLD;
    while i < lines.len() {
        print!(": ");
        stdout().flush().map_err(|e| format!("Flush error: {e}"))?;
        
        let input = read_input();
        if input == "q" {
            return Ok(())
        }
        
        // Поднимаемся на строку вверх, очищаем её и выводим новую строку
        print!("\x1b[1A");        // Вверх на одну строку
        print!("\r\x1b[2K");      // В начало строки и очистить
        println!("{}", lines[i]); // Вывести новую строку
        
        i += 1
    }
    println!(
        "{}(END){}",
        INVERT, ESC
    );

    Ok(())
}

pub fn read_input() -> String {
    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut input)
        .unwrap_or_default();
    input = input.trim().to_string();
    input
}

#[cfg(test)]
mod tests {
    use crate::terminal::print_by_line;

    #[test]
    fn unit_test_1() {
        let lines = vec![
            "Say you'll be mine, we'll be divine",
            "My paws were made for you",
            "My claws, I'll see what they can do",
            "This kitty wants her cream",
            "This pussy, oh, she has a little dream",
            "Come, let me play, don't run away",
            "I'll have my way with you",
            "And when I'm done, you'll say I slay",
            "I'm coming for you, tonight",
            "I'm ready, shut the door, turn out the light",
            "",
            "[Verse 1]",
            "Stop running for a minute, we can finally find infinity",
            "Together, we can win it all, this whole affair",
            "So come on, turn around, oh, you're so lost, but you confound me",
            "With your endless run around, it's more than I can bear",
            "You run, but in the end, I think you like this little game of hide and seek",
            "But when I find you, you won't even know your name",
            "You run, but you don't mean it, you're afraid I'll be too good",
            "In all the love you've had, you've never had it like you should",
            "Ha-ha-ha-ha-ha-ha-ha",
        ].iter().map(|s| s.to_string()).collect();

        let _ = print_by_line(lines);
    }
}