use clap::Parser;

const DEFAULT_IP: &str = "127.0.0.1:8080";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Define one or more ip addresses and port formatted as 127.0.0.1:8080
    #[arg(short, long = "addr", alias = "addresses", value_parser, num_args = 1.., value_delimiter = ' ', default_values = &[DEFAULT_IP])]
    pub addr: Vec<String>,

    /// Directory path of the server
    #[arg(short, long, default_value_t = String::from("./"))]
    pub path: String,

    /// Server entry point filename
    #[arg(short = 'i', long = "index", default_value_t = String::from("index.html"))]
    pub entry_point: String,

    /// Server not found file
    #[arg(long = "404", default_value_t = String::from("404.html"))]
    pub not_found: String,
    
    // /// Defines new path for files, exemple: hello.html:hello
    // #[arg(short,long,value_parser, num_args = 1..)]
    // pub links: Option<Vec<String>>,

    /// Config file path, icompatible with all other options
    #[arg(short, long)]
    pub config: Option<String>,
}

pub fn parse() -> Args {
    Args::parse()
}