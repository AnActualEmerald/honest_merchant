use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_tweening::TweeningPlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod input;
mod player;
mod world;

pub const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            TweeningPlugin,
        ))
        .add_plugins((InputPlugin, WorldPlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {
    // set up stuff
}
