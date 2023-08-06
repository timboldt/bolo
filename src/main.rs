#![warn(clippy::all)]

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod systems;

mod model;
use model::Player;

mod constants;
use constants::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, systems::move_player_tank)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// See https://github.com/bevyengine/bevy/blob/latest/examples/2d/rotation.rs for example.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2D orthographic camera.
    commands.spawn(Camera2dBundle::default());

    // Player controlled tank.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
            ..default()
        },
        Player {
            movement_speed: 500.0,                  // meters per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
        },
    ));
}
