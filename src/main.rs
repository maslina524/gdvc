use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gdvc")]
#[command(about = "git for Geometry Dash levels", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let cli = Cli::parse();
    
    let status: Result<(), String> = match cli.command {
        Commands::Other(args) => {
            let cmd_name = args.get(0).unwrap();
            Err(format!("gdvc: `{cmd_name}` is not a gdvc command."))
        }
    };

    if let Err(e) = status {
        eprintln!("{e}");
        std::process::exit(1);
    }
}