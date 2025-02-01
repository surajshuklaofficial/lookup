mod dns;
mod network;

use std::io::Result;
use std::net::UdpSocket;
use dns::craft::craft_dns_response;
use dns::query::{query_parser, DnsQuery};
use dns::resolver::resolver;

fn main() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:1053")?;
    println!("DNS Server listening on 127.0.0.1:1053");

    let mut buf = [0; 512];  // Buffer to store incoming DNS query

    loop {
        // Wait for an incoming query from a client
        let (len, addr) = socket.recv_from(&mut buf)?;

        // Here you should build a DNS query response for each received query
        let query = &buf[..len];

        let dns_query = match query_parser(query) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Failed to parse query: {}", e);
                continue;
            }
        };

        let DnsQuery {
            query_id,
            flags,
            question_count,
            answer_count,
            ns_count,
            ar_count,
            questions,
            ns_records,
        } = dns_query;

        println!("Received query from {} ({} bytes)", addr, len);
        println!("Query ID: 0x{:04X}", query_id);
        println!("Flags: 0x{:04X}", flags);
        println!("Questions Count: {}", question_count);
        println!("Answer Count: {}", answer_count);
        println!("NS Count: {}", ns_count);
        println!("AR Count: {}", ar_count);

        if questions.is_empty() {
            eprintln!("No questions in query, skipping...");
            continue;
        }

        println!("Questions:");
        for (i, question) in questions.iter().enumerate() {
            println!("  {}. {:?}", i + 1, question);
        }

        let domain_name = &questions[0].domain_name;
        println!("Resolving domain: {}", domain_name);

        let res = match resolver(domain_name) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to resolve {}: {}", domain_name, e);
                continue;
            }
        };

        match query_parser(&res) {
            Ok(parsed_response) => {
                let DnsQuery {
                    query_id,
                    flags,
                    question_count,
                    answer_count,
                    ns_count,
                    ar_count,
                    questions,
                    ns_records,
                } = parsed_response;

                println!("--- Parsed DNS Response ---");
                println!("Query ID: 0x{:04X}", query_id);
                println!("Flags: 0x{:04X}", flags);
                println!("Questions Count: {}", question_count);
                println!("Answer Count: {}", answer_count);
                println!("NS Count: {}", ns_count);
                println!("AR Count: {}", ar_count);

                if !questions.is_empty() {
                    println!("Questions:");
                    for (i, question) in questions.iter().enumerate() {
                        println!("  {}. Domain: {}", i + 1, question.domain_name);
                        println!("     QTYPE: 0x{:04X}, QCLASS: 0x{:04X}", question.qtype, question.qclass);
                    }
                } else {
                    println!("No questions found.");
                }

                if !ns_records.is_empty() {
                    println!("NS Records:");
                    for (i, ns) in ns_records.iter().enumerate() {
                        println!("  {}. Name: {}", i + 1, ns.ns_name);
                        println!("     Type: 0x{:04X}, Class: 0x{:04X}, TTL: {}", ns.rr_type, ns.rr_class, ns.ttl);
                        println!("     RDATA: {}", ns.rd_length);
                    }
                } else {
                    println!("No NS records found.");
                }

                println!("-----------------------------------");
            }
            Err(e) => {
                eprintln!("Failed to parse resolver response: {}", e);
                continue;
            }
        }


        let response = craft_dns_response(query)?;
        let _ = socket.send_to(&response, addr);
    }
}
