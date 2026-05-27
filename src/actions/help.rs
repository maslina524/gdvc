use std::path::PathBuf;

use std::process::Command;

pub fn run(command: Option<String>) -> Result<(), String> {
    if let Some(cmd) = command {
        cmd_handler(&cmd)?;
        return Ok(());
    }
    println!("usage: gdvc [-v | --version] [-p | --path]");
    println!("            <command> [<args>]");

    println!("\nstart a working area");
    println!("    init        Initialize your level for Gdvc");
    println!("    destroy     Remove all Gdvc tracking");
    println!("    restore     Prints and replaces the Gdvc level marker");

    println!("\nexamine the history and state");
    println!("    log         Show commit logs");
    println!("    diff        Show changes between commits");

    println!("\nwork on the current");
    println!("    commit      Record the changes");
    println!("    rollback    Restore the level to a previous commit");

    Ok(())
}

fn cmd_handler(cmd: &str) -> Result<(), String> {
    let target = "html";
    let path_str = format!("./doc/{target}/{cmd}.{target}");
    let path = PathBuf::from(&path_str);

    if !path.exists() {
        return Err(format!("fatal: '{path_str}': documentation file not found."));
    }

    #[cfg(target_os = "windows")]
    let status = Command::new("cmd").args(&["/C", "start", &path_str]).status();
    
    #[cfg(target_os = "macos")]
    let status = Command::new("open").arg(&path_str).status();
    
    #[cfg(target_os = "linux")]
    let status = Command::new("xdg-open").arg(&path_str).status();

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("failed to open file: {}", e)),
    }
}