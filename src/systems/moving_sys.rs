/* bevsteroids/src/systems/moving_sys.rs
System for entities with Velocity and Transform components
*/

use bevy::prelude::*;

use crate::components::moving::*;

/// moving_sys system - moves entities with the MovingBundle (consisting of
/// Velocity and Transform components)
/// inputs:
///     * time  - Time elapsed since last game update
///     * query - Selection of entities that have MovingBundle
pub fn moving_sys(time: Res<Time>, mut query: Query<(Entity, MovingQuery)>) {
    let delta_t = time.delta_seconds();
    for (_e, (v, mut t)) in query.iter_mut() {
        t.translation += v.velocity * delta_t;
        t.rotate(Quat::from_axis_angle(
            v.ang_vel.normalize_or_zero(),
            v.ang_vel.length() * delta_t,
        ));
    }
}
