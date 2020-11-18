use crate::attacker::client_attacker;
use crate::defender::client_defender;
use crate::types::ClientType;
use std::io::Read;
use std::net::TcpStream;

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
            _ => ClientType::Error,
        }
    }

    match client_type {
        ClientType::Attacker => client_attacker(socket),
        ClientType::Defender => client_defender(socket),
        ClientType::Error => {
            return Err(format!("Could not assign Attacker or Defender to client"))
        }
    }

    Ok(())
}
