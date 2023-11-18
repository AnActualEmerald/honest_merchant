use std::time::Duration;

use bevy::prelude::*;

#[cfg(not(target_family = "wasm"))]
use bevy::asset::LoadedFolder;
use rand::prelude::*;

use crate::{
    assets::CharacterTraits,
    utils::{
        despawn_all,
        text_box::{SpawnTextBox, TextBox},
        CalcCost, Ratios,
    },
};

use super::{
    scales::{self, ScaleContents, ScaleWeights, Submit},
    Advance, DailyGold, GameState, TargetWeight,
};

#[allow(dead_code)]
#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CustomerState {
    Greeting,
    Request,
    Measuring,
    Review,
    Reject,
    Angry,
    Payment,
    #[default]
    End,
}

#[allow(dead_code)]
#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AttentionState {
    #[default]
    Attent,
    Ignore,
}

#[derive(Resource, Default, Clone, Debug, Deref, DerefMut)]
pub struct AttentionTimer(Timer);

#[derive(Component)]
pub struct Customer(Handle<CharacterTraits>);

// don't let SBF near this
#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct CustomerAssets(Handle<LoadedFolder>);

#[cfg(target_family = "wasm")]
#[derive(Resource)]
pub struct CustomerAssets(Vec<Handle<CharacterTraits>>);

pub const CUSTOMER_STAND_POINT: Vec3 = Vec3::new(0.0, 0.0, -3.0);

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CustomerState>()
            .add_state::<AttentionState>()
            .init_resource::<TargetWeight>()
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
            .add_systems(Update, (handle_submit, wait_to_advance))
            .add_systems(OnEnter(CustomerState::Payment), pay)
            .add_systems(
                Update,
                handle_review.run_if(in_state(CustomerState::Review)),
            )
            .add_systems(Update, show_text.run_if(state_changed::<CustomerState>()))
            .add_systems(
                Update,
                despawn_all::<TextBox>.run_if(state_changed::<CustomerState>()),
            )
            .add_systems(
                OnEnter(CustomerState::End),
                (despawn_all::<Customer>, cleanup, scales::reset),
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
    // mut characters: ResMut<Assets<CharacterTraits>>,
    ass: Res<AssetServer>,
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
        Customer(ass.load("customers/dumb.chr.ron")),
    ));

    state.set(CustomerState::Greeting);
}

fn cleanup(mut tw: ResMut<TargetWeight>) {
    *tw = TargetWeight::default();
}

fn show_text(
    mut cmd: Commands,
    cust_q: Query<&Customer>,
    state: Res<State<CustomerState>>,
    chars: Res<Assets<CharacterTraits>>,
    mut spawn_text: EventWriter<SpawnTextBox>,
) {
    // .get_single wasn't working consistently here
    for char in cust_q.iter() {
        let Some(ty) = chars.get(&char.0) else {
            error!("Character traits asset was missing");
            return;
        };

        match **state {
            CustomerState::Greeting => {
                spawn_text.send(SpawnTextBox(ty.greeting.clone()));
            }
            CustomerState::Request => {
                let req_text = format!("{} please", ty.request);
                spawn_text.send(req_text.into());
                cmd.insert_resource(TargetWeight::from(&ty.request));
            }
            CustomerState::Review => {
                spawn_text.send(ty.thinking.clone().into());
            }
            CustomerState::Payment => {
                spawn_text.send(
                    format!("{} Here's {} gold", ty.accept, ty.request.customer_cost()).into(),
                );
            }
            CustomerState::Reject => {
                spawn_text.send(ty.reject.clone().into());
            }
            _ => {}
        }
    }
}

fn wait_to_advance(
    // mut cmd: Commands,
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
            CustomerState::Measuring => {
                state.set(CustomerState::Review);
            }
            CustomerState::Payment | CustomerState::Angry => {
                state.set(CustomerState::End);
            }
            CustomerState::Reject => {
                state.set(CustomerState::Measuring);
            }
            _ => {}
        }
    }
}

fn pay(mut gold: ResMut<DailyGold>, target: Res<TargetWeight>) {
    **gold += target.customer_cost();
}

fn handle_review(
    scale_weights: Res<ScaleWeights>,
    contents: Res<ScaleContents>,
    target: Res<TargetWeight>,
    mut timer: Local<Timer>,
    time: Res<Time>,
    mut state: ResMut<NextState<CustomerState>>,
) {
    if !timer.finished() {
        timer.tick(time.delta());

        if timer.just_finished() {
            info!("{contents:?} vs {target:?}");
            if scale_weights.is_even() && contents.ratio() == target.ratio() {
                state.set(CustomerState::Payment);
            } else {
                state.set(CustomerState::Reject);
            }
        }
    } else {
        *timer = Timer::new(
            Duration::from_secs_f32(thread_rng().gen_range(1.0..=3.0)),
            TimerMode::Once,
        );
    }
}

fn handle_submit(
    mut er: EventReader<Submit>,
    mut ew: EventWriter<Advance>,
    state: Res<State<CustomerState>>,
) {
    for _event in er.read() {
        if CustomerState::Measuring == **state {
            ew.send_default();
        }
    }
}
