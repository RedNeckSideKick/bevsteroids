/* bevsteroids/src/systems/bullet_sys.rs
Manages entities with Bullet components, destroying them once they age out.
*/

use bevy::prelude::*;

use crate::components::{
    bullet::*
};

/// bullet_sys system - handles bullet entities by destroying them when they age
/// out
/// inputs:
///     * cmds  - Command buffer to send entity deletion commands to
///     * time  - Time resource for updating bullet age timers
///     * query - Selects entities with Bullet components
pub fn bullet_sys(
    mut cmds: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>
) {
    let dt = time.delta();

    for (e, mut bullet) in query.iter_mut() {
        // Tick the timer and delete the bullet if it is elapsed
        if bullet.lifetime.tick(dt).just_finished() {
            cmds.entity(e).despawn();
        }
    }
}
