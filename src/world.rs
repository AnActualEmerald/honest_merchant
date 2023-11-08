use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(5.0, 2.0, 2.0).into()),
        material: materials.add(Color::RED.into()),
        ..default()
    });
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(0.5, 5.0, 5.0).into()),
        material: materials.add(Color::GOLD.into()),
        transform: Transform::from_xyz(-2.75, 2.5, 1.5),
        ..default()
    });
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(0.5, 5.0, 5.0).into()),
        material: materials.add(Color::GOLD.into()),
        transform: Transform::from_xyz(2.75, 2.5, 1.5),
        ..default()
    });
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(1000.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::CYAN,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });

    // lights
    cmd.insert_resource(AmbientLight {
        color: Color::GOLD,
        brightness: 0.02,
    });
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-1.0, 8.0, 4.0),
        ..default()
    });
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(-100.0, 100.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
