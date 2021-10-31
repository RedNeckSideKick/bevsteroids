/* bevsteroids/game.rs
This file is the primary place for central game data and logic.
*/

use bevy::prelude::*;
use bevy::render::texture::FilterMode;

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

/// texture_update_sys system - Listens to update events for Assets<Texture>
/// resources to apply any changes
/// inputs:
///     * ev_asset  - event listener tuned to updates to texture resources
///     * textures  - texture asset resource for applying changes
pub fn texture_update_sys(
    mut ev_asset: EventReader<AssetEvent<Texture>>,
    mut textures: ResMut<Assets<Texture>>
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // A texture was just loaded
                trace!("Event {:?} recieved on texture id #{:?}", ev, handle);
                let texture = textures.get_mut(handle).unwrap();

                // Apply filtering to anti-alias the texture
                texture.sampler.min_filter = FilterMode::Linear;
                texture.sampler.mag_filter = FilterMode::Linear;
            }
            AssetEvent::Modified { handle} => {
                trace!("Event {:?} recieved on texture id #{:?}", ev, handle);
            }
            AssetEvent::Removed { handle: _ } => {}
        }
    }
}
