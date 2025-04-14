use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// configuration filepath
    #[arg(short, long)]
    pub config: String,
}

pub fn parse() -> Args {
    Args::parse()
}