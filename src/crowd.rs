#![allow(dead_code)]
use bevy::prelude::*;

pub const CROWD_SIZE: u8 = 10;

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct Crowd(Vec<Entity>);

#[derive(Component)]
pub struct CrowdMember;

pub struct CrowdPlugin;

impl Plugin for CrowdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Crowd>().add_systems(Startup, spawn_crowd);
    }
}

fn spawn_crowd(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

}


