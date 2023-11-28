use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::{game::GameState, assets::Meshes};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_world);
    }
}

fn spawn_world(
    mut cmd: Commands,
    meshes: Res<Meshes>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(SceneBundle {
        scene: meshes.stand.clone(),
        ..default()
    });
    // cmd.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Box::new(5.0, 2.0, 2.0).into()),
    //     material: materials.add(Color::RED.into()),
    //     ..default()
    // });
    // cmd.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Box::new(0.5, 5.0, 5.0).into()),
    //     material: materials.add(Color::GOLD.into()),
    //     transform: Transform::from_xyz(-2.75, 2.5, 1.5),
    //     ..default()
    // });
    // cmd.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Box::new(0.5, 5.0, 5.0).into()),
    //     material: materials.add(Color::GOLD.into()),
    //     transform: Transform::from_xyz(2.75, 2.5, 1.5),
    //     ..default()
    // });
    // cmd.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(1000.0).into()),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::CYAN,
    //         perceptual_roughness: 1.0,
    //         ..default()
    //     }),
    //     ..default()
    // });

    // lights
    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    // cmd.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(-1.0, 8.0, 4.0),
    //     ..default()
    // });
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 10.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 5.0), Vec3::Y),
        ..default()
    });
}
