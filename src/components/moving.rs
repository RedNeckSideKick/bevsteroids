/* bevsteroids/src/components/moving.rs
Component for entities that need to move and rotate, e.g. the ship and asteroids
*/

use bevy::prelude::*;

/// Velocity component
/// Since this doesn't make sense to use without a transform, this base
/// component shouldn't be used directly. Instead, a bundle is used to ensure
/// the component has an associated transform.
#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec3, // velocity vector of the moving object in space
    pub ang_vel: Vec3,  // angular velocity vector about which the object rotates
}

/// Moving component bundle
/// Comes bundled with a transform because a Moving component makes no sense
/// without a transform to move
#[derive(Bundle)]
pub struct MovingBundle {
    pub v: Velocity,
    pub t: Transform,
}

/// MovingBundle component query
/// This is a shorthand for querying a Moving component bundle by wrapping a type
/// alias around the tuple of types needed for the query 
pub type MovingQuery<'a> = (&'a Velocity, &'a mut Transform);

impl MovingBundle {
    /// new_in_plane constructor - creates a MovingBundle in the xy plane
    /// inputs:
    ///     * x     - x position of transform
    ///     * y     - y position of transform
    ///     * theta - rotation about z axis of transform
    ///     * xv    - velocity in x direction
    ///     * yv    - velocity in y direction
    ///     * wz    - angular velocity in z direction
    pub fn new_in_plane(x: f32, y: f32, theta: f32, vx: f32, vy: f32, wz: f32) -> Self {
        Self {
            v: Velocity {
                velocity: Vec3::new(vx, vy, 0.0),
                ang_vel:  Vec3::new(0.0, 0.0, wz),
            },
            t: Transform {
                translation: Vec3::new(x, y, 0.0),
                rotation: Quat::from_rotation_z(theta),
                scale: Vec3::ONE,
            }
        }
    }
}
