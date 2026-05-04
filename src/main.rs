use clap::{Parser, Subcommand};

mod cmds;
mod ws;
mod level;
mod consts;
mod files;

#[derive(Parser)]
#[command(name = "gdvc")]
#[command(about = "git for Geometry Dash levels", long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = false)]
struct Cli {
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

    Help,

    #[command(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let cli = Cli::parse();
    
    let cmd = match cli.command {
        Some(c) => c,
        None => {
            cmds::help();
            std::process::exit(0);
        }
    };

    let status: Result<(), String> = match cmd {
        Commands::Init => {
            cmds::init()
        },
        Commands::Commit { message } => {
            cmds::commit(&message)
        },
        Commands::Destroy { force, _soft, hard } => {
            cmds::destroy(force, hard)
        },
        Commands::Help => {
            cmds::help();
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