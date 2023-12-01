use std::time::Duration;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::*;

use crate::{
    assets::Fonts,
    game::{GameState, Reputation, TotalExpenses, TotalGold},
    utils::{
        despawn_all,
        lenses::{BackgroundColorLens, TextLens},
        AnimatedTextBundle, Delayable, TweenDone,
    },
};

use super::PARCHMENT;

#[derive(Component)]
struct Menu;

pub struct GameEndMenuPlugin;

impl Plugin for GameEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_menu)
            .add_systems(OnExit(GameState::GameOver), despawn_all::<Menu>);
    }
}

fn spawn_menu(mut cmd: Commands) {
    let fade_in = Tween::new(
        EaseMethod::Linear,
        Duration::from_secs(1),
        BackgroundColorLens {
            start: Color::NONE,
            end: Color::BLACK,
        },
    )
    .with_completed_event(101);

    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.00),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        Menu,
        Animator::new(fade_in),
        On::<TweenDone>::run(finish_spawn),
    ));
}

fn finish_spawn(
    mut cmd: Commands,
    event: Listener<TweenDone>,
    fonts: Res<Fonts>,
    income: Res<TotalGold>,
    expenses: Res<TotalExpenses>,
    reputation: Res<Reputation>,
) {
    cmd.entity(event.target).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Px(4.0),
                    width: Val::Percent(50.0),
                    height: Val::Percent(75.0),
                    ..default()
                },
                border_color: Color::BLACK.into(),
                background_color: PARCHMENT.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Weekly Totals:",
                        TextStyle {
                            font: fonts.handwritten.clone(),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::auto(), GridTrack::fr(1.0)],
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Income: ",
                            TextStyle {
                                font: fonts.handwritten.clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        ));

                        parent.spawn((
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: fonts.default.clone(),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            Animator::new(Tween::new(
                                EaseMethod::Linear,
                                Duration::from_millis(200),
                                TextLens::new(format!("{} gold", **income)),
                            )),
                        ));

                        parent.spawn(TextBundle::from_section(
                            "Expenses: ",
                            TextStyle {
                                font: fonts.handwritten.clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        ));

                        parent.spawn((
                            TextBundle::from_sections(vec![
                                TextSection::new(
                                    "",
                                    TextStyle {
                                        font: fonts.default.clone(),
                                        font_size: 20.0,
                                        color: Color::RED,
                                    },
                                ),
                                TextSection::new(
                                    "",
                                    TextStyle {
                                        font: fonts.default.clone(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ),
                            ]),
                            Animator::new(
                                Tween::new(
                                    EaseMethod::Linear,
                                    Duration::from_millis(100),
                                    TextLens::new(format!("-{}", **expenses)),
                                )
                                .then(Tween::new(
                                    EaseMethod::Linear,
                                    Duration::from_millis(100),
                                    TextLens::new(" gold").with_section(1),
                                ))
                                .with_delay(Duration::from_millis(200)),
                            ),
                        ));
                    });

                // horizontal rule
                parent.spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(3.0),
                        width: Val::Percent(75.0),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::auto(), GridTrack::fr(1.0)],
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Total Profit: ",
                            TextStyle {
                                font: fonts.handwritten.clone(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        ));

                        parent.spawn((
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: fonts.default.clone(),
                                    font_size: 20.0,
                                    color: Color::BLACK,
                                },
                            ),
                            Animator::new(
                                Tween::new(
                                    EaseMethod::Linear,
                                    Duration::from_millis(200),
                                    TextLens::new(format!("{} gold", (**income - **expenses))),
                                )
                                .with_delay(Duration::from_millis(400)),
                            ),
                        ));
                    });

                parent
                    .spawn((AnimatedTextBundle::from_seciton_with_delay(
                        format!(
                            "Ended the week with the village feeling {} towards your stand",
                            reputation.sentiment()
                        ),
                        TextStyle {
                            font: fonts.handwritten.clone(),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                        200 * 4,
                    ),))
                    .insert(Style {
                        // align_self: AlignSelf::Start,
                        margin: UiRect::vertical(Val::Px(30.0)),
                        ..default()
                    });

                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(2.0)),
                                width: Val::Auto,
                                padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                                margin: UiRect::top(Val::Px(30.0)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            border_color: Color::BLACK.into(),
                            background_color: Color::NONE.into(),
                            ..default()
                        },
                        On::<Pointer<Down>>::run(|mut state: ResMut<NextState<GameState>>| {
                            state.set(GameState::Reset)
                        }),
                        On::<Pointer<Over>>::listener_insert(BackgroundColor(Color::ALICE_BLUE)),
                        On::<Pointer<Out>>::listener_insert(BackgroundColor(Color::NONE)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Play again",
                                TextStyle {
                                    font: fonts.default.clone(),
                                    font_size: 24.0,
                                    color: Color::BLACK,
                                },
                            ),
                            Pickable::IGNORE,
                        ));
                    });

                #[cfg(not(target_family = "wasm"))]
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(2.0)),
                                width: Val::Auto,
                                padding: UiRect::axes(Val::Px(20.), Val::Px(10.)),
                                margin: UiRect::top(Val::Px(30.0)),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            border_color: Color::BLACK.into(),
                            background_color: Color::NONE.into(),
                            ..default()
                        },
                        On::<Pointer<Down>>::run(|mut exit: EventWriter<AppExit>| {
                            exit.send_default();
                        }),
                        On::<Pointer<Over>>::listener_insert(BackgroundColor(Color::ALICE_BLUE)),
                        On::<Pointer<Out>>::listener_insert(BackgroundColor(Color::NONE)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Quit",
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
}
