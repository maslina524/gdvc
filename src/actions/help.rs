use std::env::current_exe;
use std::fs;
use std::process::Command;

use crate::terminal::print_by_line_str;

pub fn run(command: Option<String>, target: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(cmd) = command {
        cmd_handler(&cmd, &target)?;
        return Ok(());
    }
    println!("Usage: gdvc [-v | --version] [-p | --path]");
    println!("            <command> [<args>]");

    println!("\nStart a working area");
    println!("    init        Initialize your level for Gdvc");
    println!("    destroy     Remove all Gdvc tracking");
    println!("    restore     Prints and replaces the Gdvc level marker");

    println!("\nExamine the history and state");
    println!("    log         Show commit logs");
    println!("    diff        Show changes between commits");

    println!("\nWork on the current");
    println!("    commit      Record the changes");
    println!("    rollback    Restore the level to a previous commit");

    Ok(())
}

fn cmd_handler(cmd: &str, target: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let target = match target {
        Some(t) => match t.as_str() {
            "html" | "adoc" | "txt" => t,
            _ => return Err("Invalid value for target, use html, adoc, or text".into())
        },
        None => "html"
    };
    
    let binding = current_exe().unwrap();
    let path = binding.parent().unwrap().join("doc").join(target).join(format!("{cmd}.{target}"));
    let path_str = path.display().to_string();

    println!("{path_str}");

    if !path.exists() {
        return Err(format!("Fatal: '{path_str}': documentation file not found").into());
    }

    if target == "html" {
        open_html(&path_str)?
    } else {
        let doc_str = fs::read_to_string(path)
            .map_err(|e| format!("{e}"))?;
        print_by_line_str(&doc_str)?
    }

    Ok(())
}

fn open_html(path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let status = Command::new("cmd").args(&["/C", "start", &path_str]).status();
    
    #[cfg(target_os = "macos")]
    let status = Command::new("open").arg(&path_str).status();
    
    #[cfg(target_os = "linux")]
    let status = Command::new("xdg-open").arg(&path_str).status();

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open file: {}", e).into()),
    }
}