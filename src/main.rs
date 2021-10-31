/* bevsteroids/src/main.rs
An implementation of asteroids in the Bevy game engine.
This file contains only the main startup script. The majority of game data and
logic is contained in src/game.rs for organizational purposes.
*/

use bevy::prelude::*;

// Import game modules
mod game;
use game::{init, texture_update_sys};
mod components;
mod systems;
use systems::{moving_sys::*};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init.system())
        .add_system(texture_update_sys.system())    // Anti-Alias textures
        .add_system(moving_sys.system())
        .run();
}
