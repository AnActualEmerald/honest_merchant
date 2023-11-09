use bevy::prelude::*;

use crate::utils::Approx;

use super::TargetWeight;

pub const MAX_ROTATION_DEGREES: f32 = 45.0;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Weights {
    pub left: f32,
    pub right: f32,
}

pub struct ScalesPlugin;

impl Plugin for ScalesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scales)
            .add_systems(Update, update_scale_rot)
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

    cmd.spawn((
        PbrBundle {
            mesh,
            material: mat,
            transform: Transform::from_xyz(0.0, 1.25, -0.5).with_scale(Vec3::splat(0.25)),
            ..default()
        },
        Weights::default(),
    ));
}

fn set_weight(mut q: Query<&mut Weights>, w: Res<TargetWeight>) {
    for mut weights in q.iter_mut() {
        weights.right = **w;
    }
}

fn update_scale_rot(mut q: Query<(&mut Transform, &Weights)>) {
    for (mut tr, w) in q.iter_mut() {
        let total = w.left + w.right;
        if total.is_about(0.0, 0.1) {
            tr.rotation = Quat::from_rotation_z(0.0);
            continue;
        }

        let diff = w.left.max(w.right) - w.left.min(w.right);
        let dir = (w.left - w.right).signum();
        let ratio = diff / total;

        let angle_deg = MAX_ROTATION_DEGREES * ratio * dir;

        tr.rotation = Quat::from_rotation_z(angle_deg.to_radians());
    }
}
