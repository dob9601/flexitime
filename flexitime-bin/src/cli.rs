use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(help = "The date to convert to an ISO 8601 string")]
    pub date: String,
}
