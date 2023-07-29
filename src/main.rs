use bytes::{BufMut, BytesMut};
use std::io::{self, BufRead};
use tokio::net::UdpSocket;

/// This is the main function that runs our DNS resolver program.
/// It creates a socket, gets user input for the domain, then performs a DNS lookup.
#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind the socket to an ephemeral port and all available network interfaces.
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    // Using Google's public DNS server.
    let dns_server = "8.8.8.8:53";

    // Prompt the user to input a domain.
    println!("Please enter a domain name: ");
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut domain = match lines.next() {
        Some(line) => line?,
        None => return Err(io::Error::new(io::ErrorKind::Other, "No input")),
    };

    // Remove 'https://' from the domain if it's there.
    if domain.starts_with("https://") {
        domain = String::from(&domain[8..]);
    }

    // Perform a DNS lookup with the input domain.
    dns_lookup(&socket, &domain, dns_server).await?;

    Ok(())
}

/// This function builds a DNS query for a given domain.
/// The returned value is a Vec<u8> that represents the bytes of the DNS query.
fn build_query(domain: &str) -> Vec<u8> {
    let mut packet = BytesMut::with_capacity(512);

    // Header
    packet.put_u16(0x1234); // ID
    packet.put_u16(0x0100); // Flags: standard query
    packet.put_u16(1); // Questions: 1
    packet.put_u16(0); // Answers: 0
    packet.put_u16(0); // Authority RRs: 0
    packet.put_u16(0); // Additional RRs: 0

    // Question
    for part in domain.split('.') {
        packet.put_u8(part.len() as u8);
        packet.put_slice(part.as_bytes());
    }
    packet.put_u8(0); // End of domain
    packet.put_u16(1); // QTYPE: A
    packet.put_u16(1); // QCLASS: IN

    packet.to_vec()
}

/// This function parses a DNS response.
/// It takes a byte slice representing a DNS response and prints the parsed information.
fn parse_response(response: &[u8]) {
    // Header
    let id = u16::from_be_bytes([response[0], response[1]]);
    let flags = u16::from_be_bytes([response[2], response[3]]);
    let questions = u16::from_be_bytes([response[4], response[5]]);
    let answers = u16::from_be_bytes([response[6], response[7]]);
    let authority_rrs = u16::from_be_bytes([response[8], response[9]]);
    let additional_rrs = u16::from_be_bytes([response[10], response[11]]);

    println!("ID: {}", id);
    println!("Flags: {:016b}", flags);
    println!("Questions: {}", questions);
    println!("Answers: {}", answers);
    println!("Authority RRs: {}", authority_rrs);
    println!("Additional RRs: {}", additional_rrs);

    // Answer
    let mut i = 12; // Start of answer section
    for _ in 0..answers {
        while response[i] != 0 {
            i += 1;
        }
        i += 5; // Skip null byte, QTYPE, and QCLASS

        let ttl = u32::from_be_bytes([
            response[i],
            response[i + 1],
            response[i + 2],
            response[i + 3],
        ]);
        let data_len = u16::from_be_bytes([response[i + 4], response[i + 5]]);
        let ip = format!(
            "{}.{}.{}.{}",
            response[i + 6],
            response[i + 7],
            response[i + 8],
            response[i + 9]
        );

        println!("TTL: {}", ttl);
        println!("Data length: {}", data_len);
        println!("IP: {}", ip);

        i += 6 + data_len as usize;
    }
}

/// This function sends a DNS lookup request and handles the response.
/// It takes a UdpSocket, a domain, and a DNS server address.
/// It builds a DNS query for the domain, sends it to the DNS server, receives the response,
/// and then parses and prints the response.
async fn dns_lookup(socket: &UdpSocket, domain: &str, dns_server: &str) -> io::Result<()> {
    let query = build_query(domain);
    socket.send_to(&query, dns_server).await?;

    let mut buf = vec![0u8; 1024];
    let (amt, _) = socket.recv_from(&mut buf).await?;
    parse_response(&buf[..amt]);

    Ok(())
}
