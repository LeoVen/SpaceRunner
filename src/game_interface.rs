use crate::config::*;
use crate::player::{KeyState, Player};
use crate::ship::Ship;
use crate::types::ClientType;
use find_folder::Search;
use piston_window::*;

pub struct GameInterface {
    pub event: Option<Event>,
    pub window: Option<PistonWindow>,
    pub glyphs: Option<Glyphs>,
    pub client_type: ClientType,
    pub player: Player,
}

impl GameInterface {
    pub fn new(client_type: ClientType) -> Self {
        let mut window: PistonWindow = WindowSettings::new("Space Runner", Size::from(W_RES))
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let mut path = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        path.push("FiraCode-Regular.ttf");
        let glyphs = window.load_font(path).unwrap();
        let event = window.next();

        Self {
            event,
            window: Some(window),
            glyphs: Some(glyphs),
            client_type,
            player: match client_type {
                ClientType::Attacker | ClientType::Defender => Player::new(0),
                ClientType::Error => panic!("How did ClientType::Error got here??"),
            },
        }
    }
    pub fn next_event(&mut self) {
        self.event = self
            .window
            .as_mut()
            .expect("Game interface without window.")
            .next();
    }
    pub fn is_running(&self) -> bool {
        self.event.is_some()
    }
    pub fn key_pressed(&mut self) {
        // TODO send key press to server
        if let Some(event) = &self.event {
            if let Some(press_args) = event.press_args() {
                match press_args {
                    Button::Keyboard(Key::Left) => {
                        self.player.delta_left(KeyState::Pressed);
                    }
                    Button::Keyboard(Key::Right) => {
                        self.player.delta_right(KeyState::Pressed);
                    }
                    _ => {}
                }
            }
            if let Some(release_args) = event.release_args() {
                match release_args {
                    Button::Keyboard(Key::Left) => {
                        self.player.delta_left(KeyState::NotPressed);
                    }
                    Button::Keyboard(Key::Right) => {
                        self.player.delta_right(KeyState::NotPressed);
                    }
                    _ => (),
                }
            }
        }
    }
    pub fn draw(&mut self, ships: &mut [Ship], player_type: &str, score: usize) {
        // Terrible hack around the borrow checker
        let mut window = None;
        std::mem::swap(&mut self.window, &mut window);
        let mut window = window.expect("Game interface without window.");

        let mut glyphs = None;
        std::mem::swap(&mut self.glyphs, &mut glyphs);
        let mut glyphs = glyphs.expect("Game interface withouth glyphs.");

        if let Some(event) = &self.event {
            window.draw_2d(event, |c, g, d| {
                // Background
                clear(BG_COLOR, g);

                // Attacking ships
                for ship in ships.iter() {
                    for rect in ship.get_parts().iter() {
                        rectangle(SHIP_COLOR, *rect, c.transform, g);
                    }
                }

                // Player
                for rect in self.player.get_parts().iter() {
                    rectangle(PLAYER_COLOR, *rect, c.transform, g);
                }

                // Texts
                Text::new(16)
                    .draw(
                        player_type,
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(10.0, 20.0),
                        g,
                    )
                    .unwrap();
                Text::new(16)
                    .draw(
                        &format!("Score {}", score),
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(10.0, 40.0),
                        g,
                    )
                    .unwrap();

                glyphs.factory.encoder.flush(d);
            });
        }

        self.window = Some(window);
        self.glyphs = Some(glyphs);
    }
}
