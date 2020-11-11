use std::net::UdpSocket;

pub fn launch_client(addr: String) -> Result<(), String> {
    let mut socket = match UdpSocket::bind(&addr) {
        Ok(socket) => socket,
        Err(_) => return Err(format!("Failed to bind to {}", addr)),
    };

    Ok(())
}
