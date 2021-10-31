/* bevsteroids/game.rs
This file is the primary place for central game data and logic.
*/

use bevy::prelude::*;

use crate::components::moving::*;

// RESOURCES

// COMPONENTS

// SYSTEMS

/// init system - runs once at the start of the game to set everything up
/// inputs:
///     * cmds          - command buffer to send all setup commands to
///     * asset_server  - asset loader resource
///     * materials     - material resource for sprite rendering
pub fn init(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Initializing game...");

    // TODO: Load the filename from RON or at least a central constant
    let texture_handle = asset_server.load("icon.png");

    // Push resources and entities to add to the world into the command buffer
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    cmds.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    })
    // Specify initial conditions (position and velocity) for the entity
    .insert_bundle(MovingBundle::new_in_plane(0.0, 0.0, 0.0, 15.0, -5.0, 1.0));
}
