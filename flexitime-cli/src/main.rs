use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let Cli { date , format} = Cli::parse();

    let (_, result) = flexitime::parse_timestring(&date).unwrap();
    
    let chrono_time = result.to_chrono();
    if let Some(fmt) = format {
        println!("{}", chrono_time.format(&fmt))
    } else {
        println!("{}", chrono_time)
    }
}
