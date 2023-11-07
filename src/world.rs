use bevy::prelude::*;

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
        mesh: meshes.add(shape::Box::new(10.0, 2.0, 2.0).into()),
        material: materials.add(Color::RED.into()),
        ..default()
    });
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(0.5, 10.0, 10.0).into()),
        material: materials.add(Color::GOLD.into()),
        transform: Transform::from_xyz(-5.0, 5.0, 5.0),
        ..default()
    });
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
