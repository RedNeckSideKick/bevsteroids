/* bevsteroids/src/main.rs
An implementation of asteroids in the Bevy game engine.
This file contains only the main startup script. The majority of game data and
logic is contained in src/game.rs for organizational purposes.
*/

use bevy::prelude::*;

// Import game modules
mod game;
use game::{init, texture_update_sys};
mod game_config;
use game_config::GameConfig;
mod components;
mod systems;
use systems::{bullet_sys::bullet_sys, looping_sys::*, moving_sys::*, player_controller_sys::*};

fn main() {
    // Add game resources and systems
    App::build()
        .insert_resource(GameConfig::CFG)
        .add_plugins(DefaultPlugins)
        .add_startup_system(init.system())
        // TODO: bundle these into a plugin in game.rs?
        .add_system(texture_update_sys.system())    // Anti-Alias textures
        .add_system(moving_sys.system())
        .add_system(looping_sys.system())
        .add_system(player_controller_sys.system())
        .add_system(bullet_sys.system())
        .run();
}
