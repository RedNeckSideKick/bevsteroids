/* bevsteroids/src/game.rs
This file is the primary place for central game data and logic.
*/

use bevy::prelude::*;

use crate::components::{
    main_camera::*,
    moving::*,
    looping::*,
    player_controller::*,
    asteroid::*,
    ship::*,
};
use crate::game_config::GameConfig;

// RESOURCES

/// TextureHandles resource
/// Keeps track of texture handles that are needed later on in the game
#[derive(Resource)]
pub struct TextureHandles {
    pub bullet_texture: Handle<Image>,
}

// COMPONENTS

// SYSTEMS

/// init system - runs once at the start of the game to set everything up
/// inputs:
///     * cmds          - command buffer to send all setup commands to
///     * cfg           - game configuration loaded at startup
///     * asset_server  - asset loader resource
///     * materials     - material resource for sprite rendering
pub fn init(
    mut cmds: Commands,
    cfg: Res<GameConfig>,
    asset_server: Res<AssetServer>,
) {
    info!("Initializing game...");

    // TODO: Load the filename from external file (non-compiled)
    // AssetServer::load() seemed to have troubles with a non-static lifetime
    // argument being passed. This requires further investigation on my part
    // if I want to be able to pass a reference to a string slice that was
    // populated at runtime...
    let asteroid_texture = asset_server.load(cfg.asteroid);
    let ship_texture = asset_server.load(cfg.ship);
    let bullet_texture = asset_server.load(cfg.bullet);

    // Insert the texture handles into a resource so they are accessible later
    cmds.insert_resource(TextureHandles {
        bullet_texture,
    });

    // Push resources and entities to add to the world into the command buffer
    // Cameras
    cmds.spawn(Camera2dBundle::default())
        .insert(MainCamera);    // Flag this camera as the main one
    
    // TODO: wrap entity creation in functions to clean up this setup script
    // Asteroids
    cmds.spawn(SpriteBundle {
        texture: asteroid_texture.clone(),
        ..Default::default()
    })
    .insert(LoopingBundle {
        looping: Looping { radius: 128.0 },  // TODO: centrally define this
        // Specify initial conditions (position and velocity) for the entity
        moving: MovingBundle::new_in_plane(
            -500.0, 0.0, 0.0,
            15.0, -50.0, 1.0
        )
    })
    .insert(Asteroid);

    // Ship
    cmds.spawn(SpriteBundle {
        texture: ship_texture.clone(),
        ..Default::default()
    })
    .insert(LoopingBundle {
        looping: Looping { radius: 50.0 },   // TODO: centrally define this
        // Specify initial conditions (position and velocity) for the entity
        moving: MovingBundle::new_in_plane(
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0
        )
    })
    // TODO: centrally define these parameters
    .insert(PlayerController::new(250.0, 3.14, 0.08))
    .insert(Ship);
}
