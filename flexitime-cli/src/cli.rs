use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(help = "The date to convert to a particular format")]
    pub date: String,
    
    #[arg(short, long, help = "The format to convert the date to. If not specified, ISO 8601 format is used. Format specifiers are described here: https://docs.rs/chrono/latest/chrono/format/strftime/index.html")]
    pub format: Option<String>
}
