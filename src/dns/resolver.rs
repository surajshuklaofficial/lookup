use std::io::Result;
use std::net::UdpSocket;
use crate::dns::query::build_dns_query; // Assuming this function exists

pub fn resolver(domain_name: &str) -> Result<Vec<u8>> {
    let mut buf = [0; 512];

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to an available port
    socket.connect("198.41.0.4:53")?; // Connect to a root nameserver

    // Build the DNS query
    let query = build_dns_query(domain_name);

    // Send the DNS query
    socket.send(&query)?;

    // Receive the response
    let len = socket.recv(&mut buf)?;

    // Return the received bytes
    Ok(buf[..len].to_vec()) // Convert the received data to a Vec<u8>
}
