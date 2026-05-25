/// lib.rs - udp-control
/// 
/// ## How the packet looks like.
/// udp-control takes inspiration from SOCKS5 for its packets.
/// A packet only has 2 bytes. 1 for identifying the protocol and 1 for the command



pub const PROTOCOL: u8 = 0x09;
pub const GREET: u8 = 0x01;
pub const BYE: u8 = 0x11;
pub const ACK: u8 = 0x00;
pub const HEALTH_CHECK: u8 = 0xFF;
pub const FAIL: u8 = 0x55;
pub const ACK_PACKET: [u8; 2] = [PROTOCOL, ACK];
pub const ALERT: u8 = 0xAA;
