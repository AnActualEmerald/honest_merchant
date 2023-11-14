use std::time::Duration;

use bevy::{prelude::*, transform::TransformSystem};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, *};

use crate::utils::{Approx, RoundTo};

use super::TargetWeight;

pub const MAX_ROTATION_DEGREES: f32 = 45.0;
pub const WEIGHTS: [f32; 6] = [10.0, 5.0, 4.0, 3.0, 2.0, 1.0];

#[derive(Resource, Deref, Debug)]
pub struct TablePoints(Vec<Transform>);
#[derive(Resource, Deref, Debug)]
pub struct ScalePoints(Vec<Transform>);

#[derive(Event, Clone, Debug)]
pub struct AddWeight(Entity);

impl From<ListenerInput<Pointer<Down>>> for AddWeight {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self(value.target)
    }
}

#[derive(Event, Clone, Debug)]
pub struct RemoveWeight(Entity);

impl From<ListenerInput<Pointer<Down>>> for RemoveWeight {
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

#[derive(Component, Debug, Clone, Copy, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(usize);

#[derive(Component, Debug)]
pub struct OnScale;

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct ScaleWeights {
    pub left: f32,
    pub right: f32,
}

impl ScaleWeights {
    #[inline]
    pub fn is_even(&self) -> bool {
        self.left == self.right
    }
}

#[derive(Component, Debug)]
pub struct Scales;

pub struct ScalesPlugin;

impl Plugin for ScalesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddWeight>()
            .add_event::<RemoveWeight>()
            .add_event::<Submit>()
            .init_resource::<ScaleWeights>()
            .add_systems(Startup, setup_scales)
            // .add_systems(PostUpdate, place_weights.after(TransformSystem::TransformPropagate))
            .add_systems(Update, (add_weights, remove_weights, update_scale_rot))
            .add_systems(
                Update,
                set_weight.run_if(resource_exists_and_changed::<TargetWeight>()),
            );

        let mut table_points = vec![];
        for i in 0..WEIGHTS.len() {
            let points = Transform::from_xyz(
                -2.0 + (0.125 * (i + 1) as f32),
                1.0 + ((0.25 / (WEIGHTS.len() - i) as f32) / 2.0),
                -0.5,
            )
            .with_scale(Vec3::splat(0.25 / ((WEIGHTS.len() - i) as f32)));
            table_points.push(points);
        }

        app.insert_resource(TablePoints(table_points));

        let mut scale_points = vec![];
        for i in 0..WEIGHTS.len() {
            let x = (0.125 * (i + 1) as f32) % 3.0;
            let z = 0.25 - ((i as f32 % 2.0) / 2.0);
            info!("Add weight at {x:.2},{z:.2}");
            let points = Transform::from_xyz(
                -1.0 + x,
                0.5 + ((0.25 / (WEIGHTS.len() - i) as f32) / 2.0),
                z,
            )
            .with_scale(Vec3::splat(0.75 / ((WEIGHTS.len() - i) as f32)));
            scale_points.push(points);
        }
        app.insert_resource(ScalePoints(scale_points));
    }
}

pub fn reset(
    mut cmd: Commands,
    mut scale_weights: ResMut<ScaleWeights>,
    mut table_masses: Query<&mut Visibility, (With<Mass>, Without<OnScale>)>,
    mut scale_masses: Query<&mut Visibility, With<OnScale>>,

) {
    scale_weights.left = 0.0;
    scale_weights.right = 0.0;

    // cmd.entity(ent).remove_children(&masses.iter().collect::<Vec<Entity>>());
    for mut m in table_masses.iter_mut() {
        *m = Visibility::Visible;
    }

    for mut m in scale_masses.iter_mut() {
        *m = Visibility::Hidden;
    }

}

// TODO: use proper models and whatever
fn setup_scales(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    table_points: Res<TablePoints>,
    scale_points: Res<ScalePoints>
) {
    let mesh = meshes.add(Mesh::from(shape::Box::new(2.0, 1.0, 1.0)));
    let mat = materials.add(Color::BEIGE.into());
    let weight_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.5,
        height: 1.0,
        ..default()
    }));
    let weight_mat = materials.add(Color::GOLD.into());

    // spawn scales
    cmd.spawn((
        PbrBundle {
            mesh,
            material: mat,
            transform: Transform::from_xyz(0.0, 1.25, -0.5).with_scale(Vec3::splat(0.25)),
            ..default()
        },
        Scales,
    ))
    .with_children(|parent| {
        let mut shift: usize = WEIGHTS.len();
        for _ in WEIGHTS {
            shift -= 1;

            parent.spawn((
                PbrBundle {
                    mesh: weight_mesh.clone(),
                    material: weight_mat.clone(),
                    transform: scale_points[shift],
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Index(shift as usize),
                OnScale,
                On::<Pointer<Down>>::send_event::<RemoveWeight>(),
            ));
        }
    });

    //spawn weights
    let mut shift: usize = WEIGHTS.len();
    for w in WEIGHTS {
        shift -= 1;

        cmd.spawn((
            PbrBundle {
                mesh: weight_mesh.clone(),
                material: weight_mat.clone(),
                transform: table_points[shift],
                ..default()
            },
            Mass(w),
            Index(shift as usize),
            // PickableBundle::default(),
            On::<Pointer<Down>>::send_event::<AddWeight>(),
        ));
    }

    // submit bell thing
    let bell_mesh = meshes.add(
        shape::Icosphere {
            radius: 0.1,
            ..default()
        }
        .try_into()
        .unwrap(),
    );
    let bell_mat = materials.add(Color::ORANGE_RED.into());

    cmd.spawn((
        PbrBundle {
            mesh: bell_mesh,
            material: bell_mat,
            transform: Transform::from_xyz(0.75, 1.0, -0.5),
            ..default()
        },
        On::<Pointer<Down>>::send_event::<Submit>(),
    ));
}

fn add_weights(
    mut free_weights: Query<(&mut Visibility, &Index, &Mass), Without<OnScale>>,
    mut used_weights: Query<(&mut Visibility, &Index), With<OnScale>>,
    mut scale_weights: ResMut<ScaleWeights>,
    mut events: EventReader<AddWeight>,
) {
    for AddWeight(ent) in events.read() {
        if let Ok((mut vis, idx, m)) = free_weights.get_mut(*ent) {
            *vis = Visibility::Hidden;
            scale_weights.left += **m;

            info!("Num weights on scale = {}", used_weights.iter().count());
            if let Some((mut vis, _)) = used_weights
                .iter_mut()
                .find(|(_, used_idx)| used_idx.0 == idx.0)
            {
                *vis = Visibility::Visible;
            } else {
                warn!("Couldn't find weight on scale");
            }
        }
    }
}

fn remove_weights(
    mut free_weights: Query<(&mut Visibility, &Index, &Mass), Without<OnScale>>,
    mut used_weights: Query<(&mut Visibility, &Index), With<OnScale>>,
    mut scale_weights: ResMut<ScaleWeights>,
    mut events: EventReader<RemoveWeight>,
) {
    for RemoveWeight(ent) in events.read() {
        if let Ok((mut vis, idx)) = used_weights.get_mut(*ent) {
            *vis = Visibility::Hidden;

            if let Some((mut vis, _, m)) = free_weights
                .iter_mut()
                .find(|(_, used_idx, _)| used_idx.0 == idx.0)
            {
                *vis = Visibility::Visible;
                scale_weights.left -= **m;
            } else {
                warn!("Couldn't find weight on scale");
            }
        }
    }
}

fn set_weight(mut scale_weights: ResMut<ScaleWeights>, w: Res<TargetWeight>) {
    scale_weights.right = **w;
}

fn update_scale_rot(
    mut cmd: Commands,
    mut q: Query<(Entity, &Transform), With<Scales>>,
    scale_weights: Res<ScaleWeights>,
) {
    if !scale_weights.is_changed() {
        return;
    }

    for (ent, tr) in q.iter_mut() {
        let tween = if scale_weights.left > scale_weights.right {
            Tween::new(
                EaseFunction::BounceOut,
                Duration::from_millis(500),
                TransformRotationLens {
                    start: tr.rotation,
                    end: Quat::from_rotation_z(MAX_ROTATION_DEGREES.to_radians()),
                },
            )
        } else if scale_weights.left < scale_weights.right {
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
