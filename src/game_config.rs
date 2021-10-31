/* bevsteroids/src/game_config.rs
This provides the type definitions for the game_config.ron file
*/

pub struct GameConfig {
    pub asteroid: &'static str,
    pub ship: &'static str,
}

impl GameConfig {
    pub const CFG: Self = Self {
        asteroid: "icon.png",
        ship: "assets/ship.svg"
    };
}
