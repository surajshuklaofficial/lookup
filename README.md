# Lookup

A simple DNS resolver written in **Rust**, built to interact with the DNS system based on the [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt) specification.

## Overview

**`lookup`** is a fast and efficient DNS resolver that helps resolve domain names to their respective IP addresses. The project is built in Rust for maximum performance and memory safety, leveraging the power of Rust’s concurrency model to handle DNS queries reliably.

This project will eventually evolve into a full-fledged, independent DNS resolver, with the possibility of adding advanced features such as caching, support for different DNS record types, and improved error handling.

## Features

- Basic DNS query resolution.
- Supports querying **A records** (IPv4 addresses).
- Future support for additional DNS record types (e.g., **MX**, **CNAME**, etc.).
- Extensible codebase to add additional DNS features.
- High performance due to Rust's concurrency model.

## Requirements

- **Rust** (1.50 or later)
- **Linux / macOS / Windows** (platform dependent)