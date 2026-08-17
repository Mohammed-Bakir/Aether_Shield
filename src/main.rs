use tokio::net::UdpSocket;
use aether_shield::AetherWall;


#[tokio::main]
async fn main() {
    println!("=== AETHER SHIELD PROTOTYPE CORE ===");
    let mut firewall = AetherWall::new();
    let text = include_str!(".././hosts.txt");
    let socket = UdpSocket::bind("127.0.0.1:53").await.unwrap();
    println!("Successfully bound to local IPv4 loopback port 53!");

    for line in text.lines() {
        if line.starts_with("0.0.0.0 ") {
            let line = &line[8..];
            let clean = line.trim();
            firewall.insert(clean);
        }
    }

        loop {
        let mut alloc = [0u8; 512];
        
        if let Ok((bytes_received, source_address)) = socket.recv_from(&mut alloc).await {
            let mut website_name = String::new();
            let mut index = 12;

            while index < bytes_received {
                // 🛡️ SAFE BOUNDARY 1: Safely read the length byte without risking an out-of-bounds panic
                let length = match alloc.get(index) {
                    Some(&len) => len as usize,
                    None => break, // Index went past the received bytes; abort parsing cleanly
                };

                if length == 0 {
                    break;
                }
                index += 1;

                // 🛡️ SAFE BOUNDARY 2: Ensure the segment window fits inside the bytes we actually received
                if index + length <= bytes_received {
                    // 🛡️ SAFE BOUNDARY 3: Grab the slice window safely using .get()
                    if let Some(raw_bytes) = alloc.get(index..index + length) {
                        if let Ok(part) = std::str::from_utf8(raw_bytes) {
                            if !website_name.is_empty() {
                                website_name.push('.');
                            }
                            website_name.push_str(part);
                        }
                    }
                } else {
                    break; // Packet layout is malformed; drop out of parsing safely
                }

                index += length;
            }

            // Skip empty or corrupted non-DNS queries
            if website_name.is_empty() {
                continue;
            }

            // --- THE CLEAN DECISION CORE ---
            if firewall.contains(&website_name) {
                 // Drop the packet cleanly!
                 continue; 
            } else {
                // Open a standard, highly compatible IPv4 outbound socket handler
                let temp_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
                temp_socket.send_to(&alloc[0..bytes_received], "8.8.8.8:53").await.unwrap();

                let mut response_alloc = [0u8; 512];
                let (response_bytes, _) = temp_socket.recv_from(&mut response_alloc).await.unwrap();

                // Relay the exact response slice straight back to your original source endpoint
                socket.send_to(&response_alloc[..response_bytes], source_address).await.unwrap();
            }
        }
    }

}
