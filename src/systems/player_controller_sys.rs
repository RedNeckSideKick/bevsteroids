/* bevsteroids/src/systems/player_controller_sys.rs
System for entities with a PlayerController component
*/

use bevy::{
    prelude::*,
    input::{keyboard::KeyCode, Input}
};
use crate::components::{
    moving::*,
    player_controller::*,
};

/// player_controller_sys system - controls the velocity of player-controlled
/// entities
/// inputs:
///     * keyb  - Keyboard inputs resource for player control
///     * time  - Time resource for properly scaling velocity changes
///     * query - Selection of entities with a PlayerController and MovingBundle
pub fn player_controller_sys(
    keyb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&PlayerController, &mut Velocity, &Transform)>
) {
    let dt = time.delta_seconds();

    for (pc, mut v, t) in query.iter_mut() {
        let mut forward_accel = 0.0;
        let mut sideways_accel = 0.0;
        let mut yaw_accel = 0.0;

        // TODO: make these bindings configurable
        // Accelerate forward
        if keyb.pressed(KeyCode::W) {
            forward_accel += pc.accel;
        }
        // Accelerate backwards (cancel out above if both pressed)
        if keyb.pressed(KeyCode::S) {
            forward_accel -= pc.accel;
        }

        // Accelerate left
        if keyb.pressed(KeyCode::A) {
            sideways_accel += pc.accel;
        }
        // Accelerate backwards (cancel out above if both pressed)
        if keyb.pressed(KeyCode::D) {
            sideways_accel -= pc.accel;
        }

        // Accelerate counterclockwise
        if keyb.pressed(KeyCode::Q) {
            yaw_accel += pc.rot_accel;
        }
        // Accelerate clockwise (cancel out above if both pressed)
        if keyb.pressed(KeyCode::E) {
            yaw_accel -= pc.rot_accel;
        }

        // Turn commands into velocity changes
        // TODO: better rationalize initial axis vectors (X, Y, Z, or other?)
        let forward_vec = t.rotation * Vec3::Y;
        let sideways_vec = t.rotation * -Vec3::X;
        let yaw_axis_vec = Vec3::Z; // TODO: smarter calculation of this?

        v.velocity += (forward_vec * forward_accel + sideways_vec * sideways_accel) * dt;
        v.ang_vel += yaw_axis_vec * yaw_accel * dt;

        // Clamp resulting velocity magnitude to prevent excessive velocity
        // TODO: central definitions of these velocity limits
        v.velocity = v.velocity.clamp_length_max(500.0);
        v.ang_vel = v.ang_vel.clamp_length_max(12.0);
    }
}
