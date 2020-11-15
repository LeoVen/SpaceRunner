use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;
use crate::types::ClientType;

fn client_attacker(mut stream: TcpStream) {
    // Client to identify as attacker
    stream.write(&[ClientType::Attacker as u8]).unwrap();

    // while match stream.read(&mut data) {
    //     Ok(size) => {
    //         stream.write(&data[0..size]).unwrap();
    //         true
    //     },
    //     Err(_) => {
    //         false
    //     }
    // } {}
}

fn client_defender(mut stream: TcpStream) {
    // Client to identify as defender
    stream.write(&[ClientType::Defender as u8]).unwrap();

    // while match stream.read(&mut data) {
    //     Ok(size) => {
    //         stream.write(&data[0..size]).unwrap();
    //         true
    //     },
    //     Err(_) => {
    //         false
    //     }
    // } {}
}

pub fn launch_server(addr: String) -> Result<(), String> {
    let listener = match TcpListener::bind(&addr) {
        Ok(socket) => socket,
        Err(_) => return Err(format!("Failed to bind to {}", addr)),
    };

    let mut client_type = true;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if client_type {
                    thread::spawn(move|| {
                        client_defender(stream)
                    });
                } else {
                    thread::spawn(move|| {
                        client_attacker(stream)
                    });
                }
                client_type = !client_type;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
