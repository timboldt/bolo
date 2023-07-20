use bevy::prelude::*;

#[derive(Resource)]
struct HelloTimer(Timer);

fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("Hello Bevy!");
    }
}

fn main() {
    App::new()
        .insert_resource(HelloTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Update, hello_world)
        .run();
}
