use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, *};

use crate::utils::{Approx, RoundTo};

use super::TargetWeight;

pub const MAX_ROTATION_DEGREES: f32 = 45.0;
pub const MAX_WEIGHT: f32 = 100.0;

#[derive(Event, Clone, Debug)]
pub struct AddWeight(Entity);

impl From<ListenerInput<Pointer<Down>>> for AddWeight {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self(value.target)
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct Submit;

impl From<ListenerInput<Pointer<Down>>> for Submit {
    fn from(_: ListenerInput<Pointer<Down>>) -> Self {
        Self
    }
}

#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct Mass(f32);

#[derive(Component, Debug)]
pub struct OnScale;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Weights {
    pub left: f32,
    pub right: f32,
}

#[derive(Component, Debug)]
pub struct Scales;
pub struct ScalesPlugin;

impl Plugin for ScalesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddWeight>()
            .add_systems(Startup, setup_scales)
            .add_systems(Update, (add_weights, update_scale_rot))
            .add_systems(
                Update,
                set_weight.run_if(resource_exists_and_changed::<TargetWeight>()),
            );
    }
}

// TODO: use proper models and whatever
fn setup_scales(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box::new(2.0, 1.0, 1.0)));
    let mat = materials.add(Color::BEIGE.into());

    // spawn scales
    cmd.spawn((
        PbrBundle {
            mesh,
            material: mat,
            transform: Transform::from_xyz(0.0, 1.25, -0.5).with_scale(Vec3::splat(0.25)),
            ..default()
        },
        Weights::default(),
        Scales,
    ));

    let weight_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.5,
        height: 1.0,
        ..default()
    }));

    let weight_mat = materials.add(Color::GOLD.into());

    //spawn weights
    let mut shift = 1f32;
    for w in [0.5, 1.0, 2.5, 5.0, 10.0] {
        cmd.spawn((
            PbrBundle {
                mesh: weight_mesh.clone(),
                material: weight_mat.clone(),
                transform: Transform::from_xyz(-2.0 + (shift * 0.25), 1.0 + (0.25 / 2.0), -0.5)
                    .with_scale(Vec3::splat(0.25 / (6.0 - shift))),
                ..default()
            },
            Mass(w),
            // PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<AddWeight>(),
        ));
        shift += 1.0;
    }

    // submit bell thing
    let bell_mesh = meshes.add(shape::Icosphere {
        radius: 1.0,
        ..default()
    }.try_into().unwrap());
    let bell_mat = materials.add(Color::ORANGE_RED.into());

    cmd.spawn((
        PbrBundle {
            mesh: bell_mesh,
            material: bell_mat,
            transform: Transform::from_xyz(3.0, 2.0, -0.5),
            ..default()
        },
        On::<Pointer<Down>>::send_event::<Submit>(),
    ));
}

fn add_weights(
    mut cmd: Commands,
    free_weights: Query<&Mass, Without<OnScale>>,
    used_weights: Query<&Mass, With<OnScale>>,
    mut scales_q: Query<(Entity, &mut Weights)>,
    mut events: EventReader<AddWeight>,
) {
    for AddWeight(ent) in events.read() {
        if let Ok(m) = free_weights.get(*ent) {
            for (scales, mut s) in scales_q.iter_mut() {
                s.left += **m;
                cmd.entity(*ent).insert(OnScale);
                cmd.entity(scales).add_child(*ent);
            }
        } else if let Ok(m) = used_weights.get(*ent) {
            for (_, mut s) in scales_q.iter_mut() {
                s.left -= **m;
                cmd.entity(*ent).remove::<OnScale>().remove_parent();
            }
        }
    }
}

fn set_weight(mut q: Query<&mut Weights>, w: Res<TargetWeight>) {
    for mut weights in q.iter_mut() {
        weights.right = **w;
    }
}

fn update_scale_rot(
    mut cmd: Commands,
    mut q: Query<(Entity, &Transform, &Weights), Changed<Weights>>,
) {
    for (ent, tr, w) in q.iter_mut() {
        let tween = if w.left > w.right {
            Tween::new(
                EaseFunction::BounceOut,
                Duration::from_millis(500),
                TransformRotationLens {
                    start: tr.rotation,
                    end: Quat::from_rotation_z(MAX_ROTATION_DEGREES.to_radians()),
                },
            )
        } else if w.left < w.right {
            Tween::new(
                EaseFunction::BounceOut,
                Duration::from_millis(500),
                TransformRotationLens {
                    start: tr.rotation,
                    end: Quat::from_rotation_z(-MAX_ROTATION_DEGREES.to_radians()),
                },
            )
        } else {
            Tween::new(
                EaseFunction::BackOut,
                Duration::from_millis(500),
                TransformRotationLens {
                    start: tr.rotation,
                    end: Quat::from_rotation_z(0.0),
                },
            )
        };

        cmd.entity(ent).insert(Animator::new(tween));
    }
}
