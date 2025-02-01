# Learnings During DNS Resolver Project

## Big-endian vs Little-endian
- **Big-endian** stores the most significant byte first.
- **Little-endian** stores the least significant byte first.

### Example
For the number `0x12345678`:
- **Big-endian**: `0x12 0x34 0x56 0x78`
- **Little-endian**: `0x78 0x56 0x34 0x12`
...

## DNS Header Format
- The DNS header is 12 bytes.
- Fields: `ID`, `Flags`, `QDCOUNT`, `ANCOUNT`, `NSCOUNT`, `ARCOUNT`

### **List of Root DNS Servers**
There are **13 sets** of root nameservers, labeled **A to M**, each operated by different organizations. These servers are **globally distributed using Anycast** for redundancy and fast responses.

| Root Server | IPv4 Address | IPv6 Address | Operator |
|-------------|--------------|--------------|------------|
| A-root | `198.41.0.4` | `2001:503:ba3e::2:30` | Verisign |
| B-root | `199.9.14.201` | `2001:500:200::b` | USC-ISI |
| C-root | `192.33.4.12` | `2001:500:2::c` | Cogent |
| D-root | `199.7.91.13` | `2001:500:2d::d` | University of Maryland |
| E-root | `192.203.230.10` | `2001:500:a8::e` | NASA |
| F-root | `192.5.5.241` | `2001:500:2f::f` | ISC |
| G-root | `192.112.36.4` | `2001:500:12::d0d` | US DoD |
| H-root | `198.97.190.53` | `2001:500:1::53` | US Army |
| I-root | `192.36.148.17` | `2001:7fe::53` | Netnod |
| J-root | `192.58.128.30` | `2001:503:c27::2:30` | Verisign |
| K-root | `193.0.14.129` | `2001:7fd::1` | RIPE NCC |
| L-root | `199.7.83.42` | `2001:500:9f::42` | ICANN |
| M-root | `202.12.27.33` | `2001:dc3::35` | WIDE Project |

