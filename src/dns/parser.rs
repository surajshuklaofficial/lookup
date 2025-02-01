use std::io::{self, Result};

/// Parses the first 12 bytes (the header) of a DNS message.
pub fn dns_header_parser(buf: &[u8]) -> Result<(u16, u16, u16, u16)> {
    if buf.len() < 12 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid DNS query: insufficient data",
        ));
    }

    // Extract the DNS header components.
    let query_id = u16::from_be_bytes([buf[0], buf[1]]);
    let flags = u16::from_be_bytes([buf[2], buf[3]]);
    let question_count = u16::from_be_bytes([buf[4], buf[5]]);
    let answer_count = u16::from_be_bytes([buf[6], buf[7]]);

    Ok((query_id, flags, question_count, answer_count))
}
