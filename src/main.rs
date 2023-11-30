use std::time::Duration;

use assets::{AssetPlugin, Splash};
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweeningPlugin;
use crowd::CrowdPlugin;
use game::{CustomerState, GamePlugin, GameState};
use input::InputPlugin;
use player::PlayerPlugin;
use ui::{MenuState, UiPlugin};
use utils::{despawn_all, every, UtilPlugin};
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

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    Load,
    Done,
    Error,
}

#[derive(Component)]
struct SplashScreen;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "An Honest Merchant".into(),
                        resolution: WINDOW_SIZE.into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::asset::AssetPlugin {
                    processed_file_path: "assets".into(),
                    mode: AssetMode::Unprocessed,
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
        .add_systems(OnEnter(AppState::Done), setup)
        .add_systems(
            Update,
            (
                log_states,
                log_errors.run_if(
                    in_state(GameState::Error).and_then(every(Duration::from_secs_f32(0.5))),
                ),
                log_errors.run_if(
                    in_state(AppState::Error).and_then(every(Duration::from_secs_f32(0.5))),
                ),
            ),
        )
        .add_systems(OnExit(GameState::Loading), despawn_all::<SplashScreen>)
        .run();
}

fn setup(mut cmd: Commands, splash: Res<Splash>, mut state: ResMut<NextState<GameState>>) {
    info!("Spawn loading screen");
    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        SplashScreen,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Loading...",
            TextStyle {
                font: splash.font.clone(),
                font_size: 72.0,
                ..default()
            },
        ));
    });

    info!("Start loading game assets");
    state.set(GameState::Loading);
}

fn log_errors() {
    error!("Error loading assets");
}

fn log_states(// game_state: Res<State<GameState>>,
    // cust_state: Res<State<CustomerState>>,
    // menu_state: Res<State<MenuState>>,
) {
    // info!(
    //     "Program state:\n{:?}\n{:?}\n{:?}",
    //     game_state.get(),
    //     cust_state.get(),
    //     menu_state.get()
    // );
}
