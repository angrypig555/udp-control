use tokio::net::UdpSocket;
use tokio::io::{AsyncBufReadExt, BufReader};
use udp_control::{ACK_PACKET, PROTOCOL};
use std::net::SocketAddr;
use std::io::Write;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sock = Arc::new(UdpSocket::bind("0.0.0.0:5461").await?); /*{
        Ok(socket_instance) => socket_instance,
        Err(e) => {
            println!("Failed to bind to socket {}", e);
            return
        }
    };*/
    println!("[OK] Bound to port 5461");
    let mut registry = Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));
    
    let net_sock = Arc::clone(&sock);
    let net_nodes = Arc::clone(&registry);
    tokio::spawn(async move {
        let mut netbuf = [0u8; 1024];
        loop {
            if let Ok((amt, src)) = net_sock.recv_from(&mut netbuf).await {
                if amt != 2 {
                    println!("[FAIL] Got connection but the packet size was invalid");
                    continue;
                }
                let packet = &netbuf[..amt];
                if packet.is_empty() {
                    println!("[FAIL] Got connection but the packet was empty");
                    continue;
                }
                let version = packet[0];
                let command = packet[1];
                if version != PROTOCOL {
                    println!("[FAIL] Got connection but the protocol was invalid");
                    continue;
                }
                match command {
                    GREET => {
                        let mut nodes = net_nodes.lock().await;
                        
                        
                        match sock.send_to(&ACK_PACKET, src).await {
                            Ok(_) => {
                                println!("[OK] New node {} has been succesfully registered", src);
                                nodes.insert(src);
                        }
                            Err(e) => println!("[FAIL] Failed to reply to newly registered node {} {}", src, e),
                        }
                    }
                }
            }
        }
    });
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input_line = String::new();
    print!("udp-control?");
    
    loop {
        
        let result = match reader.read_line(&mut input_line).await {
            Ok(0) => break,
            Ok(_) => {
                let command = input_line.trim();
                match command  {
                    "help" => println!("udp-control - commands:\nrestart [IP] - Restarts node\nlist - Lists all the nodes"),
                    "list" => {
                        let nodes = registry.lock().await;
                        for ip in nodes.iter() {
                            println!("{ip}")
                        }
                        
                    },
                    "" => {},
                    _ => println!("unknown command")
                }
                
                
            }
            Err(e) => println!("error reading input {}", e),
        
        };
        input_line.clear();
        print!("udp-control?");
        std::io::stdout().flush()?;
        
    }
    

    Ok(())
}