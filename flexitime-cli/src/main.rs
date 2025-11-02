use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let Cli { date } = Cli::parse();

    let (_, result) = flexitime::parse_timestring(&date).unwrap();
    println!("{}", result.to_chrono())
}
