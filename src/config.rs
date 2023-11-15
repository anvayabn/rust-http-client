use clap::{Parser};
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short= 'l', long = "local", )]
    pub Local_addr: Ipv4Addr,

    #[arg(short= 'p', long= "local port", default_value_t = 8080)]
    pub Local_Port: u16,

    #[arg(short = 'd', long = "destination", )]
    pub Remote_addr: Ipv4Addr,

    #[arg(short = 'r', long = "remote port", default_value_t = 80)]
    pub Remote_port: u16,
}

impl Args {
    pub fn  new() -> Self {
        Args::parse()
    }
}