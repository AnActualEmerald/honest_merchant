use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod world;
mod player;
mod input;

pub const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 600.0) ;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }), TweeningPlugin))
        .add_plugins((InputPlugin, WorldPlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(_cmd: Commands) {
   // set up stuff
}
