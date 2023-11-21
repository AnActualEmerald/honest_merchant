use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

use crate::{
    game::{DailyExpenses, DailyGold, GameState},
    utils::{
        lenses::{BackgroundColorLens, TextLens},
        Delayable, despawn_all,
    },
};

const COMPLETED_ID: u64 = 1011;

#[derive(Component, Debug)]
struct MenuRoot;

pub struct DayEndPlugin;

impl Plugin for DayEndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DayEnd), spawn_menu)
        .add_systems(Update, spawn_modal.run_if(in_state(GameState::DayEnd)))
        .add_systems(OnExit(GameState::DayEnd), despawn_all::<MenuRoot>);
    }
}

pub fn spawn_menu(mut cmd: Commands) {
    let fade_in = Tween::new(
        EaseMethod::Linear,
        Duration::from_secs(1),
        BackgroundColorLens {
            start: Color::NONE,
            end: Color::BLACK,
        },
    )
    .with_completed_event(COMPLETED_ID);

    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(4.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Animator::new(fade_in),
    ));
}

pub fn spawn_modal(
    mut cmd: Commands,
    mut er: EventReader<TweenCompleted>,
    ass: Res<AssetServer>,
    income: Res<DailyGold>,
    expenses: Res<DailyExpenses>,
) {
    for event in er.read() {
        if event.user_data == COMPLETED_ID {
            let handwriting = ass.load("fonts/Lugrasimo-Regular.ttf");
            let font = ass.load("fonts/Inconsolata-Medium.ttf");
            cmd.entity(event.entity).with_children(|parent| {
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
                        background_color: Color::hex("#ebd5b3").unwrap().into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Daily Totals",
                                TextStyle {
                                    font: handwriting.clone(),
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
                                    grid_template_columns: vec![
                                        GridTrack::auto(),
                                        GridTrack::fr(1.0),
                                    ],
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Income: ",
                                    TextStyle {
                                        font: handwriting.clone(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ));

                                parent.spawn((
                                    TextBundle::from_sections(vec![
                                        TextSection::new(
                                            "",
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        ),
                                        TextSection::new(
                                            " gold",
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        ),
                                    ]),
                                    Animator::new(Tween::new(
                                        EaseMethod::Linear,
                                        Duration::from_millis(200),
                                        TextLens::new(income.to_string()),
                                    )),
                                ));

                                parent.spawn(TextBundle::from_section(
                                    "Expenses: ",
                                    TextStyle {
                                        font: handwriting.clone(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ));

                                parent.spawn((
                                    TextBundle::from_sections(vec![
                                        TextSection::new(
                                            "",
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 20.0,
                                                color: Color::RED,
                                            },
                                        ),
                                        TextSection::new(
                                            " gold",
                                            TextStyle {
                                                font: font.clone(),
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
                                    grid_template_columns: vec![
                                        GridTrack::auto(),
                                        GridTrack::fr(1.0),
                                    ],
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Daily Profit: ",
                                    TextStyle {
                                        font: handwriting.clone(),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ));

                                parent.spawn((
                                    TextBundle::from_sections(vec![
                                        TextSection::new(
                                            "",
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 20.0,
                                                color: Color::BLACK,
                                            },
                                        ),
                                        TextSection::new(
                                            " gold",
                                            TextStyle {
                                                font: font.clone(),
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
    }
}
