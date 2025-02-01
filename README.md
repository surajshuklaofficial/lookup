# DNS Resolver in Rust

This is a basic DNS resolver built in Rust. It resolves domain names (like `example.com`) into their corresponding IP addresses by following the DNS resolution process.

## Overview

The DNS resolver performs the following high-level steps:

1. **Check Local Cache**: If the domain has been resolved recently, the resolver will return the cached IP address.
2. **Query Root DNS Server**: If the domain is not cached, the resolver starts by asking the root DNS servers.
3. **Query TLD DNS Server**: After receiving a response from the root server, the resolver queries the TLD (Top-Level Domain) DNS server.
4. **Query Authoritative DNS Server**: The TLD server provides the authoritative DNS server, which contains the actual IP address.
5. **Return IP Address**: The authoritative DNS server returns the IP address, which is sent back to the client.
6. **Cache the Result**: The resolver caches the result for future requests.

## Flowchart

```
START
  |
  v
DNS Query Request
  |
  v
Check Local Cache 
  | 
  v
+-------------------+
| Cache Hit?        |  
+-------------------+
  |                |
  v                v
Yes                No
  |                |
  v                v
Return Cached IP   Query Root DNS Server
  |                |
  v                v
END                Query TLD DNS Server
                     |
                     v
                 Query Authoritative DNS Server
                     |
                     v
                 Return IP Address
                     |
                     v
                 Cache Result
                     |
                     v
                   END
```
## Key Terms
### DNS Header Format

A DNS header is 12 bytes long and has the following structure:

| Field    | Size (in bytes) | Description |
|----------|------------------|-------------|
| ID       | 2 bytes          | A 16-bit identifier assigned by the program to match requests and responses. |
| Flags    | 2 bytes          | Various flags indicating query/response, recursion, error codes, etc. |
| QDCOUNT  | 2 bytes          | The number of questions (queries) in the question section. |
| ANCOUNT  | 2 bytes          | The number of answer resource records. |
| NSCOUNT  | 2 bytes          | The number of name server resource records (for delegation). |
| ARCOUNT  | 2 bytes          | The number of additional resource records. |

### Breakdown of Fields

#### 1. ID (Identifier)
- **Size**: 2 bytes (16 bits)
- **Purpose**: A unique identifier assigned to the DNS request to match the request and response. Both the request and response will have the same ID.

#### 2. Flags (Flags Field)
- **Size**: 2 bytes (16 bits)
- **Purpose**: The flags field controls various aspects of the query and the response. It contains:
  - **QR (Query/Response Flag)**: 1 bit
    - `0` indicates a query.
    - `1` indicates a response.
  - **Opcode**: 4 bits (e.g., standard query, inverse query, status request).
    - `0000` for a standard query.
  - **AA (Authoritative Answer)**: 1 bit (only in responses).
    - `0` means the answer is non-authoritative.
    - `1` means the answer is authoritative.
  - **TC (Truncated)**: 1 bit
    - `0` means the message is not truncated.
    - `1` means the message is truncated (if the message size exceeds the UDP limit).
  - **RD (Recursion Desired)**: 1 bit
    - `1` means the client requests recursion.
    - `0` means no recursion requested.
  - **RA (Recursion Available)**: 1 bit
    - `1` means the server supports recursion.
    - `0` means the server does not support recursion.
  - **Z (Reserved)**: 3 bits (must be zero).
  - **RCODE (Response Code)**: 4 bits
    - `0000` = No error.
    - `0001` = Format error.
    - `0010` = Server failure.
    - `0011` = Non-existent domain (NXDOMAIN).
    - `0110` = Not implemented.
    - `0111` = Query refused.

#### 3. QDCOUNT (Question Count)
- **Size**: 2 bytes (16 bits)
- **Purpose**: Indicates the number of questions in the question section. A DNS query typically has a single question, so this value is often 1.

#### 4. ANCOUNT (Answer Count)
- **Size**: 2 bytes (16 bits)
- **Purpose**: Indicates the number of answers in the answer section. This is populated in the response to show how many resource records are provided as answers to the query.

#### 5. NSCOUNT (Name Server Count)
- **Size**: 2 bytes (16 bits)
- **Purpose**: Indicates the number of name server resource records in the authority section. These records point to authoritative name servers that can resolve the domain in question.

#### 6. ARCOUNT (Additional Record Count)
- **Size**: 2 bytes (16 bits)
- **Purpose**: Indicates the number of additional resource records in the additional section. These can include other helpful records, such as A records or other DNS records.

### DNS Query
A request to resolve a domain name into an IP address.

### Root DNS Server
The top-level DNS server that knows where to find TLD servers. There are a few root servers globally.

### TLD DNS Server
These servers manage the last portion of a domain name (e.g., `.com`, `.org`) and point to the authoritative servers.

### Authoritative DNS Server
The server that holds the actual DNS records (e.g., `A`, `AAAA`, `MX`) for a specific domain.

### A Record
Maps a domain to an IPv4 address.

### AAAA Record
Maps a domain to an IPv6 address.

### CNAME Record
Aliases one domain name to another domain.

### MX Record
Specifies the mail exchange servers for a domain.

### NS Record
Indicates the authoritative DNS servers for the domain.

### TTL (Time to Live)
The amount of time a DNS record is cached before it is considered stale.

### Recursive Query
A DNS query where the resolver continues querying servers until it finds the final answer.

## Features

- Recursive DNS query handling.
- Local cache for faster resolution.
- Support for A, AAAA, CNAME, MX, and NS DNS record types.
- Error handling for common DNS issues.
- Optionally supports DNS over HTTPS (DoH) for encrypted resolution.

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/surjshuk/lookup.git
cd lookup
cargo build --release
```

## Usage

Run the DNS resolver with a domain:

```bash
cargo run -- <domain>
```

For example:

```bash
cargo run -- example.com
```

This will resolve `example.com` and return the corresponding IP address.

## Contributing

Feel free to open an issue or submit a pull request to contribute to the project!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.