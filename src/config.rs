use clap::{Parser, Arg};
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short= 'l', long = "local", )]
    Local_addr: Ipv4Addr,

    #[arg(short= 'p', long= "local port", default_value_t = 8080)]
    Local_Port: u16,

    #[arg(short = 'd', long = "destination", )]
    Remote_addr: Ipv4Addr,

    #[arg(short = 'r', long = "remote port", default_value_t = 80)]
    Remote_port: u16,
}

impl Args {
    pub fn  new() -> Self {
        Args::parse()
    }
}