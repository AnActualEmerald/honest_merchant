use std::time::Duration;

use bevy::core_pipeline::fxaa::Fxaa;
use bevy::prelude::*;
use bevy_tweening::{
    component_animator_system, AnimationSystem, Animator, EaseFunction, Lens, Tween,
};
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::prelude::InputMap;
use leafwing_input_manager::user_input::InputKind;

use crate::WINDOW_SIZE;
use crate::input::{CursorPos, Action};

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
        .add_systems(Update, (tilt_camera_toward_mouse, look));
    }
}

fn spawn_player(mut cmd: Commands) {
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
        InputManagerBundle {
            input_map: InputMap::new([
                (InputKind::Keyboard(KeyCode::Space), Action::Advance),
                (InputKind::Mouse(MouseButton::Left), Action::Advance)
            ]),
            ..default()
        }
    ));
    // cmd.spawn(Camera2dBundle {
    //     camera_2d: Camera2d {
    //         clear_color: ClearColorConfig::None,
    //     },
    //     camera: Camera {
    //         order: 1,
    //         ..default()
    //     },
    //     ..default()
    // });
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
