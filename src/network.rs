use std::io::Result;
use std::net::UdpSocket;

/// Sends a DNS query to the given nameserver address and returns the response.
pub fn send_dns_query(query: &[u8], nameserver: &str) -> Result<Vec<u8>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // bind to any available local port
    socket.send_to(query, nameserver)?;
    
    let mut buf = [0; 512];
    let (len, _addr) = socket.recv_from(&mut buf)?;
    Ok(buf[..len].to_vec())
}
