/* bevsteroids/src/systems/player_controlled_sys.rs
System for entities with a PlayerController component
*/

use bevy::{
    prelude::*,
    input::{keyboard::KeyCode, Input},
};
use crate::{
    components::{
        moving::*,
        looping::*,
        bullet::*,
        player_controller::*,
    }, 
    game::TextureHandles,
};

/// player_controller_sys system - controls the velocity of player-controlled
/// entities
/// inputs:
///     * cmds  - Command buffer needed for spawning bullets
///     * tex   - Resource containing texture handles
///     * keyb  - Keyboard inputs resource for player control
///     * time  - Time resource for properly scaling velocity changes
///     * query - Selection of entities with a PlayerController and MovingBundle
///     * mat   - Material resource for sprite rendering
pub fn player_controller_sys(
    mut cmds: Commands,
    tex: Res<TextureHandles>,
    keyb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PlayerController, &mut Velocity, &Transform)>,
    mut mat: ResMut<Assets<ColorMaterial>>,
) {
    // Two different measures of time needed for common use and updating timer
    let dt_sec = time.delta_seconds();
    let dt = time.delta();

    for (mut pc, mut v, t) in query.iter_mut() {
        // TODO: better rationalize initial axis vectors (X, Y, Z, or other?)
        let forward_vec = t.rotation * Vec3::Y;
        let sideways_vec = t.rotation * -Vec3::X;
        let yaw_axis_vec = Vec3::Z; // TODO: smarter calculation of this?
        
        {   // Handle movement
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
            v.velocity += (forward_vec * forward_accel + sideways_vec * sideways_accel) * dt_sec;
            v.ang_vel += yaw_axis_vec * yaw_accel * dt_sec;

            // Clamp resulting velocity magnitude to prevent excessive velocity
            // TODO: central definitions of these velocity limits
            v.velocity = v.velocity.clamp_length_max(500.0);
            v.ang_vel = v.ang_vel.clamp_length_max(12.0);
        }

        {   // Handle firing
            pc.bullet_timer.tick(dt);
            if keyb.pressed(KeyCode::Space) && pc.bullet_timer.finished() {
                // Pew pew!

                // Spawn a bullet
                cmds.spawn_bundle(SpriteBundle {
                    material: mat.add(ColorMaterial {
                        texture: Some(tex.bullet_texture.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert_bundle(LoopingBundle {
                    looping: Looping { radius: 8.0 },  // TODO: centrally define this
                    // Specify initial conditions (position and velocity) for the entity
                    moving: MovingBundle {
                        v: Velocity {
                            velocity: v.velocity + forward_vec * 500.0, // TODO: centrally define bullet velocity
                            ang_vel: Vec3::ZERO,
                        },
                        // TODO: centrally define inital separation distance
                        t: Transform::from_translation(t.translation + forward_vec * 50.0),
                    }
                })
                .insert(Bullet {
                    lifetime: Timer::from_seconds(1.5, false),
                });

                pc.bullet_timer.reset();
            }
        }
    }
}
