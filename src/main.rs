/* bevsteroids/src/main.rs
An implementation of asteroids in the Bevy game engine.
This file contains only the main startup script. The majority of game data and
logic is contained in src/game.rs for organizational purposes.
*/

use bevy::prelude::*;

// Import game modules
mod game;
use game::init;
mod game_config;
use game_config::GameConfig;
mod components;
mod systems;
use systems::{bullet_sys::bullet_sys, looping_sys::*, moving_sys::*, player_controller_sys::*};

fn main() {
    // Add game resources and systems
    App::new()
        .insert_resource(GameConfig::CFG)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_linear()))
        .add_systems(Startup, init)
        // TODO: bundle these into a plugin in game.rs?
        .add_systems(Update, moving_sys)
        .add_systems(Update, looping_sys)
        .add_systems(Update, player_controller_sys)
        .add_systems(Update, bullet_sys)
        .run();
}
