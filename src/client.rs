use std::net::{TcpStream};
use std::net::Ipv4Addr;

use crate::config::Args; 

#[derive(Debug)]
pub struct HttpClient {
    local_ip_addr: Ipv4Addr,
    local_port: u16,
    remote_ip_addr: Ipv4Addr,
    remote_port: u16,
    data_size: u64,
}

impl HttpClient {
    pub fn new(args: Args) -> Self {
        HttpClient {
            local_ip_addr: args.Local_addr,
            local_port: args.Local_Port,
            remote_ip_addr: args.Remote_addr,
            remote_port: args.Remote_port,
            data_size: 64,
        }
    }
}


