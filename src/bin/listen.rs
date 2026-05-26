use std::{net::UdpSocket, net::SocketAddr};
use std::env;
use udp_control::{ACK, PROTOCOL};

use udp_control::GREET;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: listener [IP]");
        return Ok(())
    }
    let ip: SocketAddr = match args[1].parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("invalid ip: {}", e);
            return Ok(())
        }
    };
    println!("[OK] Starting listener");
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    let greeting = [PROTOCOL, GREET];
    sock.send_to(&greeting, &ip)?;
    let mut buf_greet = [0; 10];
    let (amt_greet, src_greet) = sock.recv_from(&mut buf_greet)?;
    if amt_greet != 2 {
        println!("[ERROR] server responded with incorrect packet");
        return Ok(())
    }
    let protocol = buf_greet[0];
    let message = buf_greet[1];
    if protocol != PROTOCOL {
        println!("[ERROR] server is using incorrect protocol");
        return Ok(())
    }
    if message != ACK {
        println!("[ERROR] Failed to register");
        return Ok(())
    }
    println!("[OK] Registered succesfully");
    Ok(())
}