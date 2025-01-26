use std::net::UdpSocket;

fn main() -> std::io::Result<()>{
    let servers = ["8.8.8.8:53"];

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    let query = build_dns_query("bramer.in");

    socket.send_to(&query, servers[0])?;

    let mut buf = [0; 512];

    let (amt, _) = socket.recv_from(&mut buf)?;

    println!("Query {:?}", query);
    println!("Recieved response {:?}", &buf[..amt]);

    Ok(())
} 

fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut query = vec![];

    query.extend_from_slice(&[0x23, 0x34, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    for part in domain.split('.') {
        query.push(part.len() as u8);
        query.extend_from_slice(part.as_bytes());
    }  

    query.push(0);
    query.extend_from_slice(&[0x00, 0x01]);
    query.extend_from_slice(&[0x00, 0x01]);

    query
}
