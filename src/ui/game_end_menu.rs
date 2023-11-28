use std::time::Duration;

use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use bevy_tweening::*;

use crate::{
    game::{GameState, TotalGold, TotalExpenses},
    utils::{lenses::{BackgroundColorLens, TextLens}, TweenDone, Delayable}, assets::Fonts,
};

use super::PARCHMENT;

#[derive(Component)]
struct Menu;

pub struct GameEndMenuPlugin;

impl Plugin for GameEndMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_menu);
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
                ..default()
            },
            ..default()
        },
        Menu,
        Animator::new(fade_in),
        On::<TweenDone>::run(finish_spawn),
    ));
}

fn finish_spawn(mut cmd: Commands, event: Listener<TweenDone>, fonts: Res<Fonts>, income: Res<TotalGold>, expenses: Res<TotalExpenses>) {
    cmd.entity(event.target).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(4.0)),
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    row_gap: Val::Px(4.0),
                    width: Val::Percent(50.0),
                    ..default()
                },
                border_color: Color::BLACK.into(),
                background_color: PARCHMENT.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Daily Totals",
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
                            TextBundle::from_sections(vec![
                                TextSection::new(
                                    "",
                                    TextStyle {
                                        font: fonts.default.clone(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ),
                            ]),
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
                                    " gold",
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
                                    Duration::from_millis(200),
                                    TextLens::new(format!("-{}", expenses.to_string())),
                                )
                                .with_delay(Duration::from_millis(200)),
                            ),
                        ));
                    });

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
                            "Daily Profit: ",
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
                                        color: Color::BLACK,
                                    },
                                ),
                                TextSection::new(
                                    " gold",
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
                                    Duration::from_millis(200),
                                    TextLens::new((**income - **expenses).to_string()),
                                )
                                .with_delay(Duration::from_millis(400)),
                            ),
                        ));
                    });
            });
    });
}
