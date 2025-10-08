use clap::{Parser, Subcommand};

mod prtgn_new;


#[derive(Parser)]
struct Cli {
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
            prtgn_new::new_file();
        }
        Some(Commands::Open { open_filename }) => {
            println!("Open: {}", open_filename);
            //prtgn_open::open_file(open_filename);
        }
        None => {
            println!("There was no subcommand given");
        }
    }
}
