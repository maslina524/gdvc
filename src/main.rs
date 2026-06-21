use std::env::current_exe;
use clap::{Parser, Subcommand};

use crate::consts::VERSION;

mod ws;
mod level;
mod consts;
mod files;
mod actions;
mod terminal;

#[derive(Parser)]
#[command(name = "gdvc")]
#[command(about = "Git for Geometry Dash levels", long_about = None)]
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
    Init {
        #[arg(short = 'q', long = "quiet", required = false)]
        quiet: bool,
    },

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

        #[arg(long = "amend", required = false)]
        amend: bool,
    },

    Rollback {
        #[arg(long, conflicts_with = "_hard")]
        soft: bool,
        
        #[arg(long, conflicts_with = "soft")]
        _hard: bool,

        #[arg(required = false)]
        target: String,
    },

    Restore {
        #[arg(short = 's', long = "set", required = false, conflicts_with = "gmd")]
        marker: Option<u32>,

        #[arg(long = "gmd", required = false, conflicts_with = "marker")]
        gmd: Option<String>,

        #[arg(short, long)]
        clean: bool
    },

    Log {
        #[arg(long = "oneline", required = false)]
        oneline: bool,
    },

    Diff {  },

    Help {
        #[arg(required = false)]
        command: Option<String>,

        #[arg(long = "target", short = 't', required = false)]
        target: Option<String>,
    },

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
                println!("Gdvc v{VERSION}");
            } else {
                actions::help(None, None).unwrap();
            }
            std::process::exit(0);
        }
    };

    let status: Result<(), Box<dyn std::error::Error>> = match cmd {
        Commands::Init { quiet } => {
            actions::init(quiet)
        },
        Commands::Commit { message, amend} => {
            actions::commit(&message, amend)
        },
        Commands::Rollback { target, soft, _hard } => {
            actions::rollback(target, soft)
        },
        Commands::Restore { clean, marker, gmd } => {
            actions::restore(clean, marker, gmd)
        },
        Commands::Destroy { force, _soft, hard } => {
            actions::destroy(force, hard)
        },
        Commands::Log { oneline } => {
            actions::log(oneline)
        },
        Commands::Diff {  } => {
            actions::diff()
        },
        Commands::Help { command, target } => {
            actions::help(command, target)
        },
        Commands::Other(args) => {
            let cmd_name = args.first().unwrap();
            Err(format!("Gdvc: `{cmd_name}` is not a gdvc command").into())
        }
    };

    if let Err(e) = status {
        eprintln!("{e}");
        std::process::exit(1);
    }
}