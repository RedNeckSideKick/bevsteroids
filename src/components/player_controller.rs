/* bevsteroids/src/components/player_controller.rs
Component for player-controlled entities (the ship)
*/

// use bevy::prelude::*;
// use super::moving::*;

/// PlayerController component
/// This contains information relating to the control of player-controlled
/// entities, such as their acceleration for moving and rotating.
pub struct PlayerController {
    pub accel: f32,
    pub rot_accel: f32,
}
