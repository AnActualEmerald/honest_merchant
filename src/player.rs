use std::time::Duration;

use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use bevy_tweening::{
    component_animator_system, AnimationSystem, Animator, EaseFunction, Lens, Tween,
};

use crate::game::{ScaleContents, TargetWeight, ITEM_COST};

use crate::input::CursorPos;
use crate::utils::CalcCost;
use crate::WINDOW_SIZE;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Deref, DerefMut, Debug, Default)]
pub struct LookTarget(Vec3);

pub struct LookTargetLens {
    start: Vec3,
    end: Vec3,
}

impl Lens<LookTarget> for LookTargetLens {
    fn lerp(&mut self, target: &mut LookTarget, ratio: f32) {
        **target = self.start + (self.end - self.start) * ratio;
    }
}

/// Marker to denote UI text show the current cost of the items on the scale
#[derive(Component, Debug)]
pub struct CostText;

#[derive(Component, Debug)]
pub struct CustText;

#[derive(Component, Debug)]
pub struct ProfitText;

pub const DEFAULT_LOOK: Vec3 = Vec3::new(0.0, 1.75, 0.0);
pub const DEADZONE: f32 = (WINDOW_SIZE.y / 2.0) * 0.66;
pub const LOOK_AMOUNT: f32 = 0.1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            component_animator_system::<LookTarget>.in_set(AnimationSystem::AnimationUpdate),
        )
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (tilt_camera_toward_mouse, look))
        .add_systems(
            Update,
            (
                update_cost_text.run_if(resource_changed::<ScaleContents>()),
                update_customer_text.run_if(resource_changed::<TargetWeight>()),
                update_profit_text.run_if(
                    resource_changed::<ScaleContents>().or_else(resource_changed::<TargetWeight>()),
                ),
            ),
        );
    }
}

fn spawn_player(mut cmd: Commands) {
    // player camera
    cmd.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                order: 0,
                ..default()
            },

            transform: Transform::from_xyz(0.0, 2.0, 3.0),
            ..default()
        },
        Fxaa {
            enabled: true,
            ..default()
        },
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.25, 1.0),
            falloff: FogFalloff::ExponentialSquared { density: 0.0001 },
            ..default()
        },
        Player,
        LookTarget(DEFAULT_LOOK),
    ));

    // UI stuff
    cmd.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            bottom: Val::Px(10.0),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::auto(), GridTrack::fr(1.0)],
            padding: UiRect::all(Val::Px(4.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        border_color: Color::BLACK.into(),
        background_color: Color::rgb_u8(255, 87, 51).into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Customer will pay: ",
            TextStyle::default(),
        ));

        parent.spawn((
            TextBundle {
                style: Style {
                    justify_self: JustifySelf::End,
                    ..default()
                },
                text: Text::from_section("", TextStyle::default()),
                ..default()
            },
            CustText,
        ));

        parent.spawn(TextBundle::from_section(
            "Total cost: ",
            TextStyle::default(),
        ));

        parent.spawn((
            TextBundle {
                style: Style {
                    justify_self: JustifySelf::End,
                    ..default()
                },
                text: Text::from_section("", TextStyle::default()),
                ..default()
            },
            CostText,
        ));

        parent.spawn(TextBundle::from_section(
            "Total profit: ",
            TextStyle::default(),
        ));

        parent.spawn((
            TextBundle {
                style: Style {
                    justify_self: JustifySelf::End,
                    ..default()
                },
                text: Text::from_section("", TextStyle::default()),
                ..default()
            },
            ProfitText,
        ));
    });
}

fn look(mut q: Query<(&mut Transform, &LookTarget), (With<Player>, Changed<LookTarget>)>) {
    for (mut tr, target) in q.iter_mut() {
        tr.look_at(**target, Vec3::Y);
    }
}

fn tilt_camera_toward_mouse(
    mut cmd: Commands,
    player_q: Query<(Entity, &LookTarget), With<Player>>,
    cursor: Res<CursorPos>,
) {
    if !cursor.is_changed() {
        return;
    }

    for (ent, target) in player_q.iter() {
        let new_target = if cursor.abs().x > DEADZONE && cursor.abs().y > DEADZONE {
            cursor.extend(0.0).signum() * Vec3::splat(LOOK_AMOUNT) + DEFAULT_LOOK
        } else if cursor.x < -DEADZONE {
            Vec3::new(-LOOK_AMOUNT, 0.0, 0.0) + DEFAULT_LOOK
        } else if cursor.x > DEADZONE {
            Vec3::new(LOOK_AMOUNT, 0.0, 0.0) + DEFAULT_LOOK
        } else if cursor.y > DEADZONE {
            Vec3::new(0.0, LOOK_AMOUNT, 0.0) + DEFAULT_LOOK
        } else if cursor.y < -DEADZONE {
            Vec3::new(0.0, -LOOK_AMOUNT, 0.0) + DEFAULT_LOOK
        } else {
            DEFAULT_LOOK
        };

        let lens = LookTargetLens {
            start: **target,
            end: new_target,
        };

        let tween = Tween::new(
            EaseFunction::ExponentialOut,
            Duration::from_millis(500),
            lens,
        );

        cmd.entity(ent).insert(Animator::new(tween));
    }
}

fn update_cost_text(mut q: Query<&mut Text, With<CostText>>, contents: Res<ScaleContents>) {
    for mut text in q.iter_mut() {
        let num_section = &mut text.sections[0];
        let cost: f32 = contents.cost();
        num_section.value = format!("{cost:.0} gold");
    }
}

fn update_customer_text(mut q: Query<&mut Text, With<CustText>>, target: Res<TargetWeight>) {
    for mut text in q.iter_mut() {
        let num_section = &mut text.sections[0];
        let cost: f32 = target.customer_cost();
        num_section.value = format!("{cost:.0} gold");
    }
}

fn update_profit_text(
    mut q: Query<&mut Text, With<ProfitText>>,
    contents: Res<ScaleContents>,
    target: Res<TargetWeight>,
) {
    for mut text in q.iter_mut() {
        let num_section = &mut text.sections[0];
        let profit = target.customer_cost() - contents.cost();
        num_section.value = format!("{profit:.0} gold");
    }
}
