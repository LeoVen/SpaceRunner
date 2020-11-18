use std::f64;

use serde::{Deserialize, Serialize};

use crate::config::*;
use crate::ship::*;

/// Contains only the logical part of the game
pub struct GameEngine {
    // Ships
    pub ships: Vec<Ship>,
    pub speed: f64,

    // Defender
    pub player_lane: usize,

    // Points
    pub attacker_points: usize,
    pub defender_points: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    // Enemy ships
    ships: Vec<Ship>,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            ships: vec![],
            speed: SHIP_DEFAULT_SPEED,
            player_lane: 0,
            attacker_points: 0,
            defender_points: 0,
        }
    }
    /// Makes the attacker ships fo forward
    pub fn run(&mut self, dt: f64) {
        for ship in self.ships.iter_mut() {
            ship.forward(dt);
        }
    }
}
