use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

fn hello_world() {
    println!("Hello Bevy!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                hello_world,
                // player_movement_system,
                // snap_to_player_system,
                // rotate_to_player_system,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// See https://github.com/bevyengine/bevy/blob/latest/examples/2d/rotation.rs for example.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}
