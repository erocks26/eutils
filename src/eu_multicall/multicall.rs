extern crate clap;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command{author, version, about, long_about =None}]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "eu_sleep")]
    Sleep {
        #[arg(value_name = "NUMBER[SUFFIX]", required = true)]
        duration: Vec<String> 
    },

    #[cfg(feature = "eu_true")]
    True {},
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        #[cfg(feature = "eu_sleep")]
        Some(Commands::Sleep { duration }) => {
            eu_sleep::main(duration);
        }

        #[cfg(feature = "eu_true")]
        Some(Commands::True {}) => {
            eu_true::main();
        }

        None => {
            println!("not a command");
        }
    }
}
