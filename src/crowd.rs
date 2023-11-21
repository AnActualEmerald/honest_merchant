#![allow(dead_code)]
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};
use rand::prelude::*;

pub const CROWD_SIZE: u8 = 10;

#[derive(Component)]
pub struct CrowdMember;

pub const ROAD_OFFSET: f32 = -4.125;

#[derive(Resource, Default, Deref, DerefMut, Clone, Debug)]
pub struct CrowdTextures(Vec<Handle<StandardMaterial>>);

pub struct CrowdPlugin;

impl Plugin for CrowdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrowdTextures>()
            .add_systems(Startup, spawn_crowd)
            .add_systems(Update, randomize_crowd);
    }
}

fn spawn_crowd(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<CrowdTextures>,
) {
    let mesh = meshes.add(shape::Box::new(1.0, 1.5, 0.0).into());

    for c in [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::MAROON,
        Color::ALICE_BLUE,
    ] {
        let mat = materials.add(c.into());
        textures.push(mat);
    }

    let mut rng = thread_rng();

    for i in 0..CROWD_SIZE {
        let offset = rng.gen_range(-0.5..=0.5) + ROAD_OFFSET;
        let start: f32 = *[-10.0, 10.0].choose(&mut rng).expect("????");
        let end: f32 = -10.0 * start.signum();
        let anim = Tween::new(
            EaseMethod::Linear,
            Duration::from_secs(rng.gen_range(2..=5)),
            TransformPositionLens {
                start: Vec3::new(start, 1.0, offset),
                end: Vec3::new(end, 1.0, offset),
            },
        )
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
        .with_repeat_count(RepeatCount::Infinite)
        .with_completed_event(i as u64);

        cmd.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: textures
                    .choose(&mut rng)
                    .expect("No customer textures")
                    .clone(),
                transform: Transform::from_xyz(10.0, 1.0, offset),
                ..default()
            },
            CrowdMember,
            Animator::new(anim),
        ));
    }
}

fn randomize_crowd(
    mut cmd: Commands,
    mut events: EventReader<TweenCompleted>,
    mut anims: Query<&mut Animator<Transform>, With<CrowdMember>>,
    textures: Res<CrowdTextures>,
) {
    let mut rng = SmallRng::from_entropy();
    for e in events.read() {
        let Ok(mut animator) = anims.get_mut(e.entity) else {
            continue;
        };
        animator.set_speed(rng.gen_range(0.5..=1.5));
        cmd.entity(e.entity).insert(
            textures
                .choose(&mut rng)
                .expect("Textures were empty")
                .clone(),
        );
    }
}
