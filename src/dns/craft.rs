use std::io::{self, Result};
use crate::dns::parser::dns_header_parser;

/// Crafts a DNS response based on the incoming query.
/// This example copies the question section and adds a simple A record answer.
pub fn craft_dns_response(query: &[u8]) -> Result<Vec<u8>> {
    // Parse header fields from the query.
    let (query_id, flags, question_count, _answer_count) = dns_header_parser(query)?;

    let mut response = Vec::new();

    // --- Build the DNS header (12 bytes) ---
    // Query ID (2 bytes)
    response.extend_from_slice(&query_id.to_be_bytes());

    // Flags: set QR bit to 1 (response), combine with incoming flags and set recursion available.
    let response_flags = 0x8000 | flags | 0x0080;
    response.extend_from_slice(&response_flags.to_be_bytes());

    // QDCOUNT: copy from query
    response.extend_from_slice(&question_count.to_be_bytes());

    // ANCOUNT: 1 answer
    response.extend_from_slice(&1u16.to_be_bytes());

    // NSCOUNT and ARCOUNT: 0 for this simple example.
    response.extend_from_slice(&0u16.to_be_bytes());
    response.extend_from_slice(&0u16.to_be_bytes());

    // --- Append the Question Section ---
    if query.len() < 12 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Query too short"));
    }
    let mut pos = 12;
    while pos < query.len() && query[pos] != 0 {
        pos += 1;
    }
    if pos >= query.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid query: QNAME not terminated"));
    }
    pos += 1; // include the zero terminator
    if pos + 4 > query.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Incomplete question section"));
    }
    pos += 4; // skip QTYPE and QCLASS
    let question_section = &query[12..pos];
    response.extend_from_slice(question_section);

    // --- Build the Answer Section ---
    // Use a pointer to the QNAME at offset 12 (0xC00C).
    response.extend_from_slice(&[0xC0, 0x0C]);

    // TYPE: A record (0x0001)
    response.extend_from_slice(&1u16.to_be_bytes());

    // CLASS: IN (0x0001)
    response.extend_from_slice(&1u16.to_be_bytes());

    // TTL: 4 bytes (0 seconds for simplicity)
    response.extend_from_slice(&0u32.to_be_bytes());

    // RDLENGTH: 2 bytes (IPv4 address length = 4)
    response.extend_from_slice(&4u16.to_be_bytes());

    // RDATA: IPv4 address (127.0.0.1)
    response.extend_from_slice(&[127, 0, 0, 1]);

    Ok(response)
}
