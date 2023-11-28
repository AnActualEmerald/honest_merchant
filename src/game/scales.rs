use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, *};

use crate::ui::tooltips::{TooltipBundle, TooltipText};

use super::goods::{ItemType, RemoveItem, ITEM_COLORS};

pub const MAX_ROTATION_DEGREES: f32 = 30.0;
pub const SCALE_WIDTH: f32 = 3.0;
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

#[derive(Resource, Default, Clone, Copy)]
pub struct ScaleIsSus;

#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct Mass(f32);

#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct Index(Entity);

#[derive(Component, Debug, Clone, Copy, Deref)]
pub struct Disables(Entity);

#[derive(Resource, Default, Clone, Deref, DerefMut, Debug)]
pub struct ScaleContents(HashMap<ItemType, f32>);

#[derive(Component, Debug)]
pub struct OnScale;

#[derive(Component, Debug)]
pub struct Sus;

#[derive(Event, Debug, Clone)]
pub struct SusEvent(Index);

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

pub const WEIGHT_HEIGHT: f32 = 0.125;
pub const WEIGHT_RAD: f32 = 0.05;

pub struct ScalesPlugin;

impl Plugin for ScalesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddWeight>()
            .add_event::<RemoveWeight>()
            .add_event::<Submit>()
            .add_event::<SusEvent>()
            .init_resource::<ScaleWeights>()
            .init_resource::<ScaleContents>()
            .add_systems(Startup, setup_scales)
            // .add_systems(PostUpdate, place_weights.after(TransformSystem::TransformPropagate))
            .add_systems(
                Update,
                (add_weights, remove_weights, update_scale_rot, scale_piles, update_sus.run_if(resource_changed::<ScaleWeights>())),
            )
            .add_systems(
                Update,
                (set_weight).run_if(resource_exists_and_changed::<ScaleContents>()),
            );

        let mut table_points = vec![];
        for i in 0..WEIGHTS.len() {
            let scale = Vec3::splat((i as f32).powf(0.25));
            let points = Transform::from_xyz(
                -2.0 + (WEIGHT_RAD * 2.0 + 0.1) * (i as f32),
                1.0 + (WEIGHT_HEIGHT * scale.y) / 2.0,
                -0.5,
            )
            .with_scale(scale);
            table_points.push(points);
        }

        app.insert_resource(TablePoints(table_points.clone()));

        let mut scale_points = vec![];
        let mut row = -0.5;
        for (i, tr) in table_points.iter().enumerate().take(WEIGHTS.len()) {
            if i as f32 % 3.0 == 0.0 {
                row += 0.5;
            }
            let x = 0.3 * (i as f32 % 3.0);
            let z = 0.5 - row;
            let scale = tr.scale * 2.0;
            info!("Add weight at {x:.2},{z:.2}");
            let points = Transform::from_xyz(
                (-SCALE_WIDTH - (WEIGHT_RAD)) / 2.0 + (x),
                0.5 + ((WEIGHT_HEIGHT * scale.y) / 2.0),
                z,
            )
            .with_scale(scale);
            scale_points.push(points);
        }
        app.insert_resource(ScalePoints(scale_points));
    }
}

pub fn reset(
    mut scale_weights: ResMut<ScaleWeights>,
    mut contents: ResMut<ScaleContents>,
    mut table_masses: Query<&mut Visibility, (With<Mass>, Without<OnScale>)>,
    mut scale_masses: Query<&mut Visibility, (With<OnScale>, Without<ItemType>)>,
) {
    scale_weights.left = 0.0;
    scale_weights.right = 0.0;
    *contents = ScaleContents::default();

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
    scale_points: Res<ScalePoints>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box::new(3.0, 1.0, 1.0)));
    let mat = materials.add(Color::BEIGE.into());
    let weight_mesh = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.05,
        height: 0.125,
        ..default()
    }));
    let weight_mat = materials.add(Color::GOLD.into());
    let sus_mat = materials.add(Color::ORANGE_RED.into());

    let mut on_scale = vec![];
    let mut on_scale_sus = vec![];

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
        for m in WEIGHTS {
            shift -= 1;

            on_scale.push(
                parent
                    .spawn((
                        PbrBundle {
                            mesh: weight_mesh.clone(),
                            material: weight_mat.clone(),
                            transform: scale_points[shift],
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        OnScale,
                        TooltipBundle::new(format!("{m} grams")),
                        On::<Pointer<Down>>::send_event::<RemoveWeight>(),
                    ))
                    .id(),
            );
        }

        let mut shift: usize = WEIGHTS.len();
        for m in WEIGHTS {
            shift -= 1;

            on_scale_sus.push(
                parent
                    .spawn((
                        PbrBundle {
                            mesh: weight_mesh.clone(),
                            material: sus_mat.clone(),
                            transform: scale_points[shift],
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        OnScale,
                        Sus,
                        TooltipBundle::new(format!("{} grams", m / 2.0)),
                        On::<Pointer<Down>>::send_event::<RemoveWeight>(),
                    ))
                    .id(),
            );
        }

        let mut row = -1.0;
        for (i, col) in ITEM_COLORS.iter().enumerate() {
            let Some(t) = ItemType::from_repr(i) else {
                continue;
            };

            if i as f32 % 2.0 == 0.0 {
                row += 1.0;
            }

            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(
                        shape::Icosphere {
                            radius: 0.25,
                            ..default()
                        }
                        .try_into()
                        .unwrap(),
                    ),
                    material: materials.add((*col).into()),
                    transform: Transform::from_xyz(
                        SCALE_WIDTH / 2.0 - 0.25 - (i as f32 % 2.0) / 2.0,
                        0.5,
                        -0.25 + (row / 2.0),
                    )
                    .with_scale(Vec3::ZERO),
                    ..default()
                },
                t,
                OnScale,
                TooltipBundle::new("0 grams"),
                On::<Pointer<Down>>::send_event::<RemoveItem>(),
            ));
        }
    });

    let mut off_scale = vec![];
    let mut off_scale_sus = vec![];

    //spawn weights
    let mut shift: usize = WEIGHTS.len();
    for (i, w) in WEIGHTS.iter().enumerate() {
        shift -= 1;

        let on_scale = on_scale[i];
        let disable = on_scale_sus[i];

        let ent = cmd
            .spawn((
                PbrBundle {
                    mesh: weight_mesh.clone(),
                    material: weight_mat.clone(),
                    transform: table_points[shift],
                    ..default()
                },
                Mass(*w),
                Index(on_scale),
                Disables(disable),
                TooltipBundle::new(format!("{w} grams")),
                // PickableBundle::default(),
                On::<Pointer<Down>>::send_event::<AddWeight>(),
            ))
            .id();

        cmd.entity(on_scale).insert(Index(ent));
        off_scale.push(ent);
    }

    // the sus weights
    let mut shift: usize = WEIGHTS.len();
    for (i, w) in WEIGHTS.iter().enumerate() {
        shift -= 1;

        let translation = table_points[shift].translation + Vec3::new(0.0, -0.5, 0.5);
        let disable = on_scale[i];
        let on_scale = on_scale_sus[i];

        let ent = cmd
            .spawn((
                PbrBundle {
                    mesh: weight_mesh.clone(),
                    material: sus_mat.clone(),
                    transform: table_points[shift].with_translation(translation),
                    ..default()
                },
                Mass(w / 2.0),
                Index(on_scale),
                Disables(disable),
                TooltipBundle::new(format!("{} grams", w / 2.0)),
                Sus,
                On::<Pointer<Down>>::send_event::<AddWeight>(),
            ))
            .id();

        cmd.entity(on_scale).insert(Index(ent));
        off_scale_sus.push(ent);
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
    mut free_weights: Query<
        (&mut Visibility, &Index, &Mass, &Disables, Option<&Sus>),
        Without<OnScale>,
    >,
    mut used_weights: Query<&mut Visibility, With<OnScale>>,
    mut scale_weights: ResMut<ScaleWeights>,
    mut events: EventReader<AddWeight>,
    mut sus_events: EventWriter<SusEvent>,
    mut remove_weight: EventWriter<RemoveWeight>,
) {
    for AddWeight(ent) in events.read() {
        if let Ok((mut vis, idx, m, disable, sus)) = free_weights.get_mut(*ent) {
            *vis = Visibility::Hidden;
            scale_weights.left += **m;

            info!("Num weights on scale = {}", used_weights.iter().count());

            if Ok(&Visibility::Visible) == used_weights.get(disable.0) {
                remove_weight.send(RemoveWeight(disable.0));
            }

            if let Ok(mut vis) = used_weights.get_mut(idx.0) {
                *vis = Visibility::Visible;
            } else {
                warn!("Couldn't find weight on scale");
            }

            if sus.is_some() {
                sus_events.send(SusEvent(*idx));
            }
        }
    }
}

fn remove_weights(
    mut free_weights: Query<(&mut Visibility, &Index, &Mass), (Without<OnScale>, Without<Sus>)>,
    mut free_sus_weights: Query<(&mut Visibility, &Index, &Mass), (Without<OnScale>, With<Sus>)>,

    mut used_weights: Query<(&mut Visibility, &Index, Option<&Sus>), With<OnScale>>,
    mut scale_weights: ResMut<ScaleWeights>,
    mut events: EventReader<RemoveWeight>,
) {
    for RemoveWeight(ent) in events.read() {
        if let Ok((mut vis, idx, sus)) = used_weights.get_mut(*ent) {
            *vis = Visibility::Hidden;

            let off_scale = if sus.is_some() {
                free_sus_weights.get_mut(idx.0)
            } else {
                free_weights.get_mut(idx.0)
            };

            if let Ok((mut vis, _, m)) = off_scale {
                *vis = Visibility::Visible;
                scale_weights.left -= **m;
            } else {
                warn!("Couldn't find weight on scale");
            }
        }
    }
}

fn set_weight(mut scale_weights: ResMut<ScaleWeights>, contents: Res<ScaleContents>) {
    let weight = contents.values().sum();
    scale_weights.right = weight;
}

fn scale_piles(
    mut q: Query<(&mut Transform, &mut TooltipText, &ItemType), With<OnScale>>,
    contents: Res<ScaleContents>,
) {
    for (mut tr, mut txt, ty) in q.iter_mut() {
        let scale = *contents.get(ty).unwrap_or(&0.0);
        tr.scale = Vec3::splat(scale.powf(0.25));
        txt.0 = format!("{scale} grams");
    }
}

fn update_sus(q: Query<&Visibility, (With<Sus>, With<OnScale>)>, mut cmd: Commands) {
    let sus_count = q.iter().filter(|vis| **vis != Visibility::Hidden).count();
    if sus_count > 0 {
        cmd.init_resource::<ScaleIsSus>();
    } else {
        cmd.remove_resource::<ScaleIsSus>();
    }
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
