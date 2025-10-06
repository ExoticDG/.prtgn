use clap::{Parser, Subcommand};
use std::path::PathBuf;

//mod prtgn_new;



#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        new_filename: String,
    },
    Open {
        open_filename: String,
    },
}

pub fn command() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::New { new_filename }) => {
            println!("Create: {:?}", new_filename);
        }
        Some(Commands::Open { open_filename }) => {
            println!("Open: {}", open_filename);
        }
        None => {
            println!("There was no subcommand given");
        }
    }
}
