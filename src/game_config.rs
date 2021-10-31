/* bevsteroids/src/game_config.rs
This provides the type definitions for the game_config.ron file
*/

pub struct GameConfig {
    pub asteroid: &'static str,
}

impl GameConfig {
    pub const CFG: Self = Self {
        asteroid: "icon.png",
    };
}
