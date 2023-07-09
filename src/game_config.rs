/* bevsteroids/src/game_config.rs
This provides the type definitions for the game_config.ron file
*/

use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub asteroid: &'static str,
    pub ship: &'static str,
    pub bullet: &'static str,
}

impl GameConfig {
    pub const CFG: Self = Self {
        asteroid: "icon.png",
        ship: "ship.png",
        bullet: "bullet.png",
    };
}
