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
    #[command(about = "Create or edit a .prtgn file")]
    Init {
        filename: String,
    },

}

pub fn command() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { filename }) => {
            println!("Create: {:?}", filename);
            prtgn_init::new_file(filename.to_string());
        }
        // Some(Commands::Open { open_filename }) => {
        //     println!("Open: {}", open_filename);
        //     //prtgn_open::open_file(open_filename);
        // }
        None => {
            println!("There was no subcommand given");
        }
    }
}
