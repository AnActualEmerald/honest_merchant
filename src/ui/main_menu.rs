use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    assets::{Fonts, Images},
    game::GameState,
    utils::despawn_all,
};

use super::PARCHMENT;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct Help;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, States, PartialEq, Eq, Hash, Default)]
enum MainMenuState {
    #[default]
    Menu,
    About,
    Help,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_systems(OnEnter(MainMenuState::Help), show_about)
            .add_systems(OnExit(MainMenuState::Help), despawn_all::<Help>)
            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_all::<MainMenu>);
    }
}

fn show_about(mut cmd: Commands, images: Res<Images>) {
    cmd.spawn((
        ImageBundle {
            image: images.example.clone().into(),
            ..default()
        },
        Help,
        On::<Pointer<Down>>::run(|mut state: ResMut<NextState<MainMenuState>>| {
            state.set(MainMenuState::Menu)
        }),
    ));
}

fn spawn_menu(mut cmd: Commands, fonts: Res<Fonts>) {
    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        MainMenu,
    ))
    .with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(80.0),
                    border: UiRect::all(Val::Px(2.0)),
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    // row_gap: Val::Px(30.0),
                    grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0)],
                    ..default()
                },
                border_color: Color::BLACK.into(),
                background_color: PARCHMENT.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "An Honest Merchant",
                        TextStyle {
                            font: fonts.handwritten.clone(),
                            font_size: 44.0,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        border: UiRect::all(Val::Px(2.0)),
                                        width: Val::Auto,
                                        padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    border_color: Color::BLACK.into(),
                                    background_color: Color::NONE.into(),
                                    ..default()
                                },
                                On::<Pointer<Down>>::run(
                                    |mut state: ResMut<NextState<GameState>>| {
                                        state.set(GameState::DayStart)
                                    },
                                ),
                                On::<Pointer<Over>>::listener_insert(BackgroundColor(
                                    Color::ALICE_BLUE,
                                )),
                                On::<Pointer<Out>>::listener_insert(BackgroundColor(Color::NONE)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "Play",
                                        TextStyle {
                                            font: fonts.default.clone(),
                                            font_size: 24.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    Pickable::IGNORE,
                                ));
                            });

                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        border: UiRect::all(Val::Px(2.0)),
                                        width: Val::Auto,
                                        padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                                        ..default()
                                    },
                                    border_color: Color::BLACK.into(),
                                    background_color: Color::NONE.into(),
                                    ..default()
                                },
                                On::<Pointer<Down>>::run(
                                    |mut state: ResMut<NextState<MainMenuState>>| {
                                        state.set(MainMenuState::Help)
                                    },
                                ),
                                On::<Pointer<Over>>::listener_insert(BackgroundColor(
                                    Color::ALICE_BLUE,
                                )),
                                On::<Pointer<Out>>::listener_insert(BackgroundColor(Color::NONE)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        "How To Play",
                                        TextStyle {
                                            font: fonts.default.clone(),
                                            font_size: 24.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    Pickable::IGNORE,
                                ));
                            });
                    });
            });
    });
}
