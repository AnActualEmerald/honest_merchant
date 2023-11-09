use bevy::{asset::LoadedFolder, prelude::*};
use serde::Deserialize;

use crate::{assets::Characteristics, utils::spawn_text_box};

use super::GameState;

#[allow(dead_code)]
#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InteractionState {
    Greeting,
    Request,
    Measuring,
    Weighing,
    Payment,
    #[default]
    End,
}

#[derive(Component)]
pub struct Customer(Characteristics);

// don't let SBF near this
#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct CustomerAssets(Handle<LoadedFolder>);

#[cfg(target_family = "wasm")]
#[derive(Resource)]
pub struct CustomerAssets(Vec<Handle<Characteristics>>);

pub const CUSTOMER_STAND_POINT: Vec3 = Vec3::new(0.0, 0.0, -3.0);

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InteractionState>()
            .add_systems(Startup, |mut cmd: Commands, ass: Res<AssetServer>| {
                // preload (or try to preload) all the customer character files
                #[cfg(not(target_family = "wasm"))]
                cmd.insert_resource(CustomerAssets(ass.load_folder("customers")));

                // this is stupid but I get it
                #[cfg(target_family = "wasm")]
                {
                    let mut res = vec![];
                    for c in ["dumb"] {
                        res.push(ass.load(format!("customers/{c}.chr.ron")));
                    }
                    cmd.insert_resource(CustomerAssets(res));
                }
            })
            .add_systems(OnEnter(GameState::Customer), spawn_customer)
            .add_systems(OnEnter(InteractionState::Greeting), show_greeting);
    }
}

// TODO: This should animate in a customer from the crowd
fn spawn_customer(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<NextState<InteractionState>>,
) {
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.0,
                depth: 2.0,
                ..default()
            })),
            material: materials.add(Color::INDIGO.into()),
            transform: Transform::from_translation(CUSTOMER_STAND_POINT + Vec3::new(0.0, 1.0, 0.0)),
            ..default()
        },
        Customer(Characteristics {
            name: "dumb".into(),
            greeting: vec!["Wonderful weather we're having".into()],
            request: super::ItemType::SpiderEyes(10.0),
            attention_type: super::AttentionType::Distracted,
        }),
    ));

    state.set(InteractionState::Greeting);
}

fn show_greeting(mut cmd: Commands, cust_q: Query<&Customer>) {
    let Customer(ty) = cust_q.single();
    spawn_text_box(&mut cmd, &ty.greeting[0]);
}
