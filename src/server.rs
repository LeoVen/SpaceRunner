use crate::types::ClientType;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::defender::server_defender;
use crate::attacker::server_attacker;
use std::sync::{Arc, Mutex};
use crate::game_engine::GameEngine;

pub fn launch_server(addr: String) -> Result<(), String> {
    let listener = match TcpListener::bind(&addr) {
        Ok(socket) => socket,
        Err(_) => return Err(format!("Failed to bind to {}", addr)),
    };

    let mut client_type = true;
    let mut handles = vec![];
    let mut game_engine = Arc::new(Mutex::new(GameEngine::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let game = Arc::clone(&game_engine);
                if client_type {
                    stream.write(&[ClientType::Defender as u8]).unwrap();
                    let handle = thread::spawn(move || server_defender(stream, game));
                    handles.push(handle);
                } else {
                    stream.write(&[ClientType::Attacker as u8]).unwrap();
                    let handle = thread::spawn(move || server_attacker(stream, game));
                    handles.push(handle);
                    // Create another game engine every second iteration
                    game_engine = Arc::new(Mutex::new(GameEngine::new()));
                }
                client_type = !client_type;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
