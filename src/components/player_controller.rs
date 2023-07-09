/* bevsteroids/src/components/player_controlled.rs
Component for player-controlled entities (the ship)
*/

use bevy::{
    prelude::*,
    utils::Duration,
};
// use super::moving::*;

/// PlayerControlled component
/// This contains information relating to the control of player-controlled
/// entities: their acceleration for moving and rotating, cooldown for firing
/// projectiles.
#[derive(Component)]
pub struct PlayerController {
    pub accel: f32,
    pub rot_accel: f32,
    pub bullet_timer: Timer,
}

impl PlayerController {
    pub fn new(accel: f32, rot_accel: f32, bullet_cooldown: f32) -> Self {
        // Create a timer that has already elapsed fully
        let mut timer = Timer::from_seconds(bullet_cooldown, TimerMode::Once);
        timer.tick(Duration::from_secs_f32(bullet_cooldown));
        Self {
            accel,
            rot_accel,
            bullet_timer: timer
        }
    }
}
