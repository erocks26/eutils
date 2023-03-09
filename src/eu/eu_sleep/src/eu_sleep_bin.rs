mod eu_sleep_lib;

use clap::{Parser};

#[derive(Parser)]
#[command(name = "eu_sleep")]
#[command(author = "Ethan R. <ethanraque@protonmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "todo", long_about = None)]
struct Cli {
    duration: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    eu_sleep_lib::main(cli.duration);
}
