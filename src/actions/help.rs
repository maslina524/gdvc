pub fn run() {
    println!("usage: gdvc [-v | --version] [-p | --path]");
    println!("            <command> [<args>]");

    println!("\nstart a working area");
    println!("    init        Initialize your level for Gdvc");
    println!("    destroy     Remove all Gdvc tracking");

    println!("\nexamine the history and state");
    println!("    log         Show commit logs");

    println!("\nwork on the current");
    println!("    commit      Record the changes");
    println!("    rollback    Restore the level to a previous commit");
}