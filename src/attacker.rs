use crate::game_interface::GameInterface;
use crate::types::ClientType;
use std::net::TcpStream;
use crate::game_engine::GameEngine;
use std::sync::{Arc, Mutex};

pub fn client_attacker(socket: TcpStream) {
    let mut game = GameInterface::new(ClientType::Attacker);

    while game.is_running() {
        game.key_pressed();

        game.next_event();
    }
}

pub fn server_attacker(mut stream: TcpStream, game_engine: Arc<Mutex<GameEngine>>) {
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
