mod sleep_lib;

use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    long_about = "Pause for NUMBER seconds. SUFFIX may be 's' for seconds (the default),\n'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an\ninteger.  Given two or more arguments, pause for the amount of time\nspecified by the sum of their values."
)]
struct Cli {
    #[arg(value_name = "NUMBER[SUFFIX]", required = true)]
    duration: Vec<String>,
}

fn main() {
    sleep_lib::main(Cli::parse().duration);
}
