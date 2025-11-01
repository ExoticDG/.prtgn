use clap::{Parser, Subcommand};

mod prtgn_init;

#[derive(Parser)]
#[command(author, version, about = "
    A protogen inspired file extension written in Rust.

    .prtgn  Copyright (C) 2025  ExoticDG
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.

    Licensed under the GNU General Public License v3.0
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new .prtgn file")]
    Init {
        filename: String,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        wav: bool,
    },


}

pub fn command() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { filename, wav }) => {
            println!("Create: {:?}", filename);
            prtgn_init::init(filename.to_string(), *wav);
        }
        // Some(Commands::Open { filename }) => {
        //     println!("Open: {}", filename);
        //     prtgn_open::open_file(filename.to_string());a
        //     //prtgn_open::open_file(open_filename);
        // }
        None => {
            println!("There was no subcommand given");
        }
    }
}
