use std::net::TcpStream;
use std::io::Read;
use crate::types::ClientType;

pub fn launch_client(addr: String) -> Result<(), String> {
    let mut socket = match TcpStream::connect(&addr) {
        Ok(socket) => socket,
        Err(_) => return Err(format!("Failed to connect to {}", addr)),
    };

    let mut client_type_data = [0 as u8; 1];

    let mut client_type = ClientType::Error;

    if let Ok(_) = socket.read(&mut client_type_data) {
        client_type = match client_type_data[0] {
            1 => ClientType::Attacker,
            2 => ClientType::Defender,
            _ => ClientType::Error
        }
    }

    println!("{:?}", client_type);

    Ok(())
}
