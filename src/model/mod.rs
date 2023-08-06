#![warn(clippy::all)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    // Linear speed in something/s.
    pub movement_speed: f32,
    // Rotation speed in rad/s.
    pub rotation_speed: f32,
}