# Rust-DNS-Resolver
This repository hosts a simple DNS resolver implemented in Rust using the Tokio library for asynchronous network programming. The program constructs DNS queries, sends them to a DNS server, and parses the responses to display relevant DNS information.

## Getting Started
Prerequisites:

You need to have Rust installed on your machine. Check the Rust documentation for installation instructions.
You also need to have the Tokio and Bytes crates. These will be installed automatically when you run the program, as they are specified in the Cargo.toml file.
Installation:

Clone this repository to your local machine.
```
git clone https://github.com/yourusername/Rust-DNS-Resolver.git
```
Go into the cloned directory and run the program. 
```
cargo run
```

## How it Works
When you run the program, it will prompt you to input a domain. The domain name is used to construct a DNS query, which is sent to Google's DNS server (8.8.8.8). The program then receives the DNS server's response and parses it to extract the relevant DNS information, such as the queried domain's IP address.

If you input a domain that starts with 'https://', the program will automatically remove it before processing the domain.

The parsed DNS information is then displayed, including the following:
- ID: The identification of the DNS query.
- Flags: The flags in the DNS header.
- Questions: The number of questions in the DNS query.
- Answers: The number of answers in the DNS response.
- Authority RRs: The number of authority resource records in the DNS response.
- Additional RRs: The number of additional resource records in the DNS response.
- TTL: The time-to-live of the answer.
- Data length: The data length of the answer.
- IP: The IP address of the domain.

## License
MIT
