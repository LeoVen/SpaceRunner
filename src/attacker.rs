use crate::game_interface::GameInterface;
use crate::types::ClientType;
use std::net::TcpStream;

pub fn client_attacker(socket: TcpStream) {
    let mut game = GameInterface::new(ClientType::Attacker);

    while game.is_running() {
        game.key_pressed();

        game.next_event();
    }
}
