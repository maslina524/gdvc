use std::env::current_exe;
use clap::{Parser, Subcommand};

use crate::consts::VERSION;

mod ws;
mod level;
mod consts;
mod files;
mod actions;

#[derive(Parser)]
#[command(name = "gdvc")]
#[command(about = "git for Geometry Dash levels", long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = false)]
struct Cli {
    #[arg(short = 'v', long = "version", global = true, conflicts_with = "path")]
    version: bool,

    #[arg(short = 'p', long = "path", global = true, conflicts_with = "version")]
    path: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,

    Destroy {
        #[arg(short = 'f', long = "force", required = false)]
        force: bool,

        #[arg(long, conflicts_with = "hard")]
        _soft: bool,
        
        #[arg(long, conflicts_with = "_soft")]
        hard: bool,
    },

    Commit {
        #[arg(short = 'm', long = "message", required = true)]
        message: String,
    },

    Rollback {
        #[arg(required = false)]
        target: String,
    },

    Log {
        #[arg(long = "oneline", required = false)]
        oneline: bool,
    },

    Help,

    #[command(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let cli = Cli::parse();
    
    let cmd = match cli.command {
        Some(c) => c,
        None => {
            if cli.path {
                let path = current_exe().unwrap().display().to_string();
                println!("{path}");
            } else if cli.version {
                println!("gdvc v{VERSION}");
            } else {
                actions::help::run();
            }
            std::process::exit(0);
        }
    };

    let status: Result<(), String> = match cmd {
        Commands::Init => {
            actions::init::run()
        },
        Commands::Commit { message } => {
            actions::commit::run(&message)
        },
        Commands::Rollback { target } => {
            actions::rollback::run(target)
        },
        Commands::Destroy { force, _soft, hard } => {
            actions::destroy::run(force, hard)
        },
        Commands::Log { oneline } => {
            actions::log::run(oneline)
        },
        Commands::Help => {
            actions::help::run();
            Ok(())
        },
        Commands::Other(args) => {
            let cmd_name = args.first().unwrap();
            Err(format!("gdvc: `{cmd_name}` is not a gdvc command."))
        }
    };

    if let Err(e) = status {
        eprintln!("{e}");
        std::process::exit(1);
    }
}