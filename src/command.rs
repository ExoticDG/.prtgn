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
        #[arg(long)]
        outdir: PathBuf,
    },
    Open {
        #[arg(long)]
        to: String,
    },
}

pub fn command() {
    let args = Cli::parse();

    println!("root: {:?}", args.root);

    match &args.command {
        Some(Commands::New { outdir }) => {
            println!("outdir: {:?}", outdir);
        }
        Some(Commands::Open { to }) => {
            println!("to: {}", to);
        }
        None => {
            println!("There was no subcommand given");
        }
    }
}
