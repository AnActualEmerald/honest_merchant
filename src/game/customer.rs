use bevy::{asset::LoadedFolder, prelude::*};


use crate::{
    assets::Characteristics,
    utils::{despawn_all, spawn_text_box, text_box::TextBox},
};

use super::{Advance, GameState, TargetWeight};

#[allow(dead_code)]
#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CustomerState {
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
        app.add_state::<CustomerState>()
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
            .add_systems(Update, wait_to_advance)
            .add_systems(Update, show_text.run_if(state_changed::<CustomerState>()))
            .add_systems(
                Update,
                despawn_all::<TextBox>.run_if(state_changed::<CustomerState>()),
            )
            .add_systems(OnEnter(GameState::Customer), spawn_customer);
    }
}

// TODO: This should animate in a customer from the crowd
fn spawn_customer(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<NextState<CustomerState>>,
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

    state.set(CustomerState::Greeting);
}

fn show_text(mut cmd: Commands, cust_q: Query<&Customer>, state: Res<State<CustomerState>>) {
    let Ok(Customer(ty)) = cust_q.get_single() else {
        return;
    };
    match **state {
        CustomerState::Greeting => {
            spawn_text_box(&mut cmd, &ty.greeting[0]);
        }
        CustomerState::Request => {
            let req_text = format!("{} please", ty.request);
            spawn_text_box(&mut cmd, req_text);
            cmd.insert_resource(TargetWeight::from(ty.request));
        }
        _ => {}
    }
}

fn wait_to_advance(
    current_state: Res<State<CustomerState>>,
    mut state: ResMut<NextState<CustomerState>>,
    mut er: EventReader<Advance>,
) {
    for _event in er.read() {
        match **current_state {
            CustomerState::Greeting => {
                state.set(CustomerState::Request);
            }
            CustomerState::Request => {
                state.set(CustomerState::Measuring);
            }
            _ => {}
        }
    }
}
