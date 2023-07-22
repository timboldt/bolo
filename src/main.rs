use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
struct Player {
    // Linear speed in something/s.
    movement_speed: f32,
    // Rotation speed in rad/s.
    rotation_speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_movement_system,))
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

// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (tank, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * tank.rotation_speed * TIME_STEP);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * tank.movement_speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}
