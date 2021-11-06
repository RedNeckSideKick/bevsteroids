/* bevsteroids/src/components/looping.rs
Component for entities that loop at the screen borders
*/

use bevy::prelude::*;
use super::moving::*;

/// Looping component and bundle bundle
/// Similar to how the Moving component doesn't make sense without a transform
/// component, this component doesn't make sense without a Moving component.
/// Same story, don't use this directly and instead use a bundle.
pub struct Looping {
    pub radius: f32,
}

#[derive(Bundle)]
pub struct LoopingBundle {
    pub looping: Looping,
    #[bundle]
    pub moving: MovingBundle,
}

/// Looping bundle query
/// This is a shorthand for querying a Looping component bundle by wrapping a
/// type alias around the tuple of types needed for the query 
pub type LoopingQuery<'a> = (&'a Looping, MovingQuery<'a>);
