/* bevsteroids/src/main.rs
An implementation of asteroids in the Bevy game engine.
This file contains only the main startup script. The majority of game data and
logic is contained in src/game.rs for organizational purposes.
*/

use bevy::prelude::*;

// Import game modules
mod game;
use game::init;
mod components;
mod systems;
use systems::{moving_sys::*};

fn main() {
    App::build()
        // TODO: Figure out how to get AA to actually work
        // .insert_resource(Msaa { samples: 4 })   // Anti-aliasing
        .add_plugins(DefaultPlugins)
        .add_startup_system(init.system())
        .add_system(moving_sys.system())
        .run();
}
