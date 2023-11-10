use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use bevy_mod_picking::prelude::*;
use game::GamePlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use utils::UtilPlugin;
use world::WorldPlugin;
use assets::AssetPlugin;


mod input;
mod player;
mod world;
mod crowd;
mod game;
mod assets;
mod utils;

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
            DefaultPickingPlugins,
        ))
        .add_plugins((UtilPlugin, AssetPlugin, InputPlugin, WorldPlugin, GamePlugin, PlayerPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands) {
    // set up stuff
}
