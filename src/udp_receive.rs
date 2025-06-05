use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use crate::udp_decode::DataType;
use crate::udp_decode::decode_data;

pub fn udp_connectable(ips: &[[u8; 4]], ports: &[u16]) -> Vec<SocketAddr> {
    let mut results = Vec::new();
    
    for ip in ips {
        for port in ports {
            let socket_addr = SocketAddr::from((*ip, *port));
            let is_connectable = check_single_address(socket_addr);
            if is_connectable {
                results.push(socket_addr);
            }
        }
    }
    
    results
}

fn check_single_address(addr: SocketAddr) -> bool {
    match UdpSocket::bind(addr) {
        Ok(socket) => {
            // Set read timeout to 100ms (similar to Python's 0.1 seconds)
            if socket.set_read_timeout(Some(Duration::from_millis(100))).is_err() {
                return false;
            }
            
            let mut buf = [0u8; 1500];
            
            // Try to receive data with timeout
            match socket.recv_from(&mut buf) {
                Ok((_size, _addr)) => true,  // Data received
                Err(_) => false,             // Timeout or error
            }
            // Socket is automatically closed when it goes out of scope
        },
        Err(_) => false, // Couldn't bind to address
    }
}

pub fn connect(sock: &Vec<SocketAddr>) -> UdpSocket {
    UdpSocket::bind(&sock[..]).expect("Failed to connect")
}

pub fn receive(socket: &UdpSocket) -> Result<HashMap<String, DataType>, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 1500];

    let (size, _addr) = socket.recv_from(&mut buf)?;
    // println!("Received {} size from {}", size, _addr);

    let returned_data = &buf[..size];

    let processed_data = decode_data(&returned_data);

    Ok(processed_data)
}