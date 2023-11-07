use bevy::prelude::*;
use world::WorldPlugin;

mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((WorldPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 10.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    });
}
