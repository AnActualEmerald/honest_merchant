use bevy::prelude::*;

use crate::{assets::Fonts, game::{TargetWeight, GameState}, utils::Total};

use super::{NeedsTextSet, PARCHMENT};

#[derive(Component, Debug)]
struct OrderTicket;

#[derive(Component, Debug)]
struct OrderGrid;

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_ui).add_systems(
            Update,
            update_ticket
                .run_if(resource_exists_and_changed::<TargetWeight>())
                .in_set(NeedsTextSet),
        );
    }
}

fn spawn_ui(mut cmd: Commands, fonts: Res<Fonts>) {
    // "order ticket"
    cmd.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Percent(20.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                padding: UiRect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            border_color: Color::BLACK.into(),
            background_color: PARCHMENT.into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        OrderTicket,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Current Order",
            TextStyle {
                font: fonts.handwritten.clone(),
                font_size: 20.0,
                color: Color::BLACK,
            },
        ));

        // grid for listing requested items
        parent.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::auto()],
                    ..default()
                },
                ..default()
            },
            OrderGrid,
        ));

        // // horizontal rule
        // parent.spawn(NodeBundle {
        //     style: Style {
        //         height: Val::Px(3.0),
        //         width: Val::Percent(75.0),
        //         ..default()
        //     },
        //     background_color: Color::BLACK.into(),
        //     ..default()
        // });

        // //
    });
}

fn update_ticket(
    mut cmd: Commands,
    mut vis_q: Query<&mut Visibility, With<OrderTicket>>,
    q: Query<Entity, With<OrderGrid>>,
    target: Res<TargetWeight>,
    fonts: Res<Fonts>,
) {
    if let Ok(mut vis) = vis_q.get_single_mut() {
        *vis = if target.total() > 0.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }

    for ent in q.iter() {
        cmd.entity(ent)
            .despawn_descendants()
            .with_children(|parent| {
                for (t, amnt) in target.iter() {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{t} - "),
                            TextStyle {
                                font: fonts.handwritten.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            format!("{amnt:.1} g"),
                            TextStyle {
                                font: fonts.handwritten.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        ),
                        style: Style {
                            justify_self: JustifySelf::End,
                            ..default()
                        },
                        ..default()
                    });
                }
            });
    }
}
