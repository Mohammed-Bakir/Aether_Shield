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
            // ⏳ Start your high-resolution telemetry clock immediately
            let start_time = std::time::Instant::now();

            let mut website_name = String::new();
            let mut index = 12;

            while index < bytes_received {
                let length = match alloc.get(index) {
                    Some(&len) => len as usize,
                    None => break,
                };

                if length == 0 {
                    break;
                }
                index += 1;

                if index + length <= bytes_received {
                    if let Some(raw_bytes) = alloc.get(index..index + length) {
                        if let Ok(part) = std::str::from_utf8(raw_bytes) {
                            if !website_name.is_empty() {
                                website_name.push('.');
                            }
                            website_name.push_str(part);
                        }
                    }
                } else {
                    break;
                }

                index += length;
            }

            if website_name.is_empty() {
                continue;
            }

            if firewall.contains(&website_name) {
                 let duration = start_time.elapsed();
                 println!("[❌ DROPPED] {} | Trie Match: {:?}", website_name, duration);
                 continue; 
            } else {
                let temp_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
                temp_socket.send_to(&alloc[0..bytes_received], "8.8.8.8:53").await.unwrap();

                let mut response_alloc = [0u8; 512];
                let (response_bytes, _) = temp_socket.recv_from(&mut response_alloc).await.unwrap();

                socket.send_to(&response_alloc[..response_bytes], source_address).await.unwrap();

                let total_duration = start_time.elapsed();
                println!("[✅ ALLOWED] {} | Proxy Latency: {:?}", website_name, total_duration);
            }
        }
    }


}
