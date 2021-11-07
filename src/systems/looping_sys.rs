/* bevsteroids/src/systems/looping_sys.rs
System for entities with Looping (+ Moving & Transform) components
*/

use bevy::prelude::*;

use crate::components::{
    main_camera::*,
    looping::*
};

/// looping_sys system - loop Moving entities when they reach the edge of the
/// screen
/// inputs:
///     * wnds  - Windows resource, used for finding the window edges
///     * query - Query set including looping entities and main camera transform
pub fn looping_sys(
    wnds: Res<Windows>,
    // mut query: Query<LoopingQuery>,
    mut query: QuerySet<(
        Query<LoopingQuery>,
        Query<&Transform, With<MainCamera>>
    )>
) {
    // Get primary window sizing
    let wnd = wnds.get_primary().unwrap();
    // NOTE: assuming the default orthographic projection (pixels from center)
    // hence divide by 2
    let wnd_x_dist = wnd.width() / 2.0;
    let wnd_y_dist = wnd.height() / 2.0;

    // Get the transform of the main camera
    let cam_tf = query.q1().single().unwrap();
    // NOTE: assuming the camera is not rotated or off-axis in any way, i.e.
    // camera x-axis aligned with world x-axis, same for y, z
    
    // Find window boundaries in coordinate space
    let x_max = cam_tf.translation.x + wnd_x_dist;
    let x_min = cam_tf.translation.x - wnd_x_dist;
    let y_max = cam_tf.translation.y + wnd_y_dist;
    let y_min = cam_tf.translation.y - wnd_y_dist;
    
    for (looping, (_, mut t)) in query.q0_mut().iter_mut() {
        // Loop x axis
        if t.translation.x > x_max + looping.radius {
            t.translation.x -= (wnd_x_dist + looping.radius) * 2.0;
        } else if t.translation.x < x_min - looping.radius {
            t.translation.x += (wnd_x_dist + looping.radius) * 2.0;
        }

        // Loop y axis
        if t.translation.y > y_max + looping.radius {
            t.translation.y -= (wnd_y_dist + looping.radius) * 2.0;
        } else if t.translation.y < y_min - looping.radius {
            t.translation.y += (wnd_y_dist + looping.radius) * 2.0;
        }
    }
}
