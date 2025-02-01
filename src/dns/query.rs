use std::io::{self, Result};

#[derive(Debug)]
pub struct DnsQuery {
    pub query_id: u16,
    pub flags: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
    pub questions: Vec<Question>,
    pub ns_records: Vec<NsRecord>,
}

#[derive(Debug)]
pub struct Question {
    pub domain_name: String,
    pub qtype: u16,
    pub qclass: u16,
}

#[derive(Debug)]
pub struct NsRecord {
    pub rr_type: u16,
    pub rr_class: u16,
    pub ttl: u32,
    pub ns_name: String,
    pub rd_length: usize,
}

/// Builds a basic DNS query for a given domain name,
/// requesting NS records (QTYPE = 2, QCLASS = 1).
pub fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut query = Vec::new();

    // Transaction ID (for example: 0x1234)
    query.extend_from_slice(&[0x12, 0x34]);

    // Flags: Standard query (0x0100)
    query.extend_from_slice(&[0x01, 0x00]);

    // QDCOUNT: 1 question
    query.extend_from_slice(&[0x00, 0x01]);

    // ANCOUNT, NSCOUNT, ARCOUNT: 0 (no answers provided in the query)
    query.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    // Encode the domain name in QNAME format.
    for label in domain.split('.') {
        query.push(label.len() as u8);
        query.extend_from_slice(label.as_bytes());
    }
    query.push(0x00); // Terminate QNAME

    // QTYPE: NS (0x0002)
    query.extend_from_slice(&[0x00, 0x02]);

    // QCLASS: IN (0x0001)
    query.extend_from_slice(&[0x00, 0x01]);

    query
}

pub fn query_parser(buf: &[u8]) -> Result<DnsQuery> {
    println!("{:?}", buf);

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
    let ns_count = u16::from_be_bytes([buf[8], buf[9]]);
    let ar_count = u16::from_be_bytes([buf[10], buf[11]]);

    let mut pos = 12;
    let mut questions = Vec::new();
    let mut ns_records = Vec::new();

    // --- Parse the Question Section ---
    for _ in 0..question_count {
        let qname = read_domain_name(buf, &mut pos);
        let qtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let qclass = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]);
    
        questions.push(Question {
            domain_name: qname,
            qtype,
            qclass,
        });

        pos += 4; // skip QTYPE and QCLASS
    }

    // --- Parse the Answer Section (if any) ---
    for _ in 0..answer_count {
        let _name = read_domain_name(buf, &mut pos);
        let rdlength = u16::from_be_bytes([buf[pos + 8], buf[pos + 9]]) as usize;
        pos += 10; // skip fixed fields: type (2), class (2), TTL (4), RDLENGTH (2)
        pos += rdlength; // skip RDATA
    }

    // --- Parse the NS records (if any) ---
    for _ in 0..ns_count {
        // Read the NAME field (often a pointer)
        let _auth_name = read_domain_name(buf, &mut pos);
        let rr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let rr_class = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]);
        let ttl = u32::from_be_bytes([
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        let rd_length = u16::from_be_bytes([buf[pos + 8], buf[pos + 9]]) as usize;
        pos += 10; // Move past the fixed fields

        // For NS records, RDATA is a domain name.
        let ns_name = read_domain_name(buf, &mut pos);
        ns_records.push(NsRecord {
            rr_type,
            rr_class,
            ttl,
            ns_name,
            rd_length
        });
    }
    
    // Return the populated DnsQuery struct
    Ok(DnsQuery {
        query_id,
        flags,
        question_count,
        answer_count,
        ns_count,
        ar_count,
        questions,
        ns_records,
    })
}

/// Reads a domain name from the DNS response, handling compression (pointers).
pub fn read_domain_name(response: &[u8], pos: &mut usize) -> String {
    let mut name = String::new();
    let mut jumped = false;
    let mut offset = *pos;

    loop {
        let len = response[offset];
        // End of name?
        if len == 0 {
            offset += 1;
            break;
        }
        // Check for pointer (two highest bits set)
        if len & 0xC0 == 0xC0 {
            let b2 = response[offset + 1];
            let pointer = (((len & 0x3F) as u16) << 8) | b2 as u16;
            if !jumped {
                *pos = offset + 2;
            }
            offset = pointer as usize;
            jumped = true;
            continue;
        } else {
            offset += 1;
            let label = &response[offset..offset + (len as usize)];
            name.push_str(&String::from_utf8_lossy(label));
            name.push('.');
            offset += len as usize;
        }
    }
    if !jumped {
        *pos = offset;
    }
    name.trim_end_matches('.').to_string()
}
