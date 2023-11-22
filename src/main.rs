use assets::AssetPlugin;
use bevy::prelude::*;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweeningPlugin;
use crowd::CrowdPlugin;
use game::{CustomerState, GamePlugin, GameState};
use input::InputPlugin;
use player::PlayerPlugin;
use ui::{MenuState, UiPlugin};
use utils::UtilPlugin;
use world::WorldPlugin;

mod assets;
mod crowd;
mod game;
mod input;
mod player;
mod ui;
mod utils;
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
            DefaultPickingPlugins,
            BillboardPlugin,
        ))
        .add_plugins((
            UtilPlugin,
            AssetPlugin,
            CrowdPlugin,
            InputPlugin,
            WorldPlugin,
            GamePlugin,
            PlayerPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, log_states)
        .run();
}

fn setup() { //mut cmd: Commands) {
             // set up stuff
}

fn log_states(
    game_state: Res<State<GameState>>,
    cust_state: Res<State<CustomerState>>,
    menu_state: Res<State<MenuState>>,
) {
    info!(
        "Program state:\n{:?}\n{:?}\n{:?}",
        game_state.get(),
        cust_state.get(),
        menu_state.get()
    );
}
