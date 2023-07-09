/* bevsteroids/src/components/bullet.rs
Component for bullet projectile entities
*/

use bevy::prelude::*;

/// Bullet component
/// Holds the timer for a bullet's lifetime (before it self-destructs)
#[derive(Component)]
pub struct Bullet {
    pub lifetime: Timer,
}
