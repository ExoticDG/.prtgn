use clap::{Parser, Subcommand};

mod prtgn_init;


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        filename: String,
    },
    // Open {
    //     open_filename: String,
    // },
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
