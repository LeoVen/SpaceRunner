use std::net::TcpStream;
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};

use crate::game_interface::GameInterface;
use crate::types::ClientType;
use crate::game_engine::GameEngine;
use crate::ship::Ship;
use crate::config::SHIP_DEFAULT_SPEED;

pub fn client_defender(mut stream: TcpStream) {
    let mut game = GameInterface::new(ClientType::Attacker);

    while game.is_running() {
        game.key_pressed();

        let mut data = Box::new([0 as u8; 10000]);
        stream.read(&mut *data);
        println!("{}", std::str::from_utf8(&*data).expect("Data is not UTF-8"));

        game.next_event();
        break;
    }
}

pub fn server_defender(mut stream: TcpStream, mut game_engine: Arc<Mutex<GameEngine>>) {

    (*game_engine).lock().expect("Mutex lock failed").ships.push(Ship::new(0, SHIP_DEFAULT_SPEED));

    let data = serde_json::to_string(&(*game_engine).lock().expect("Mutex lock failed").ships).unwrap();
    stream.write(data.as_bytes());

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
