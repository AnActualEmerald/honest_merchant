use std::time::Duration;

use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};

use crate::{
    assets::CharacterTraits,
    utils::{despawn_all, spawn_text_box, text_box::TextBox},
};

use super::{
    goods::ItemType,
    scales::{self, ScaleWeights, Submit, ScaleContents},
    Advance, GameState, TargetWeight,
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
    mut characters: ResMut<Assets<CharacterTraits>>,
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
        Customer(characters.add(CharacterTraits {
            name: "dumb".into(),
            greeting: vec!["Wonderful weather we're having".into()],
            thinking: "Hmm...".into(),
            accept: "Ah, perfect!".into(),
            reject: "Uh, no. I don't think that's right.".into(),
            accuse: "Hey! What are you trying to pull!".into(),
            request: super::ItemRequest(HashMap::from([(ItemType::SpiderEyes, 10.0)])),
            attention_type: super::AttentionType::Distracted,
            max_gold: 100,
        })),
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
) {
    // .get_single wasn't working consistently here
    for char in cust_q.iter() {
        let Some(ty) = chars.get(&char.0) else {
            error!("Character traits asset was missing");
            return;
        };

        match **state {
            CustomerState::Greeting => {
                spawn_text_box(&mut cmd, &ty.greeting[0]);
            }
            CustomerState::Request => {
                let req_text = format!("{} please", ty.request);
                spawn_text_box(&mut cmd, req_text);
                cmd.insert_resource(TargetWeight::from(&ty.request));
            }
            CustomerState::Review => {
                spawn_text_box(&mut cmd, &ty.thinking);
            }
            CustomerState::Payment => {
                spawn_text_box(
                    &mut cmd,
                    format!("{} Here's {} gold", ty.accept, ty.max_gold),
                );
            }
            CustomerState::Reject => {
                spawn_text_box(&mut cmd, &ty.reject);
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
            if scale_weights.is_even() && **contents == **target {
                state.set(CustomerState::Payment);
            } else {
                state.set(CustomerState::Reject);

            }
        }
    } else {
        *timer = Timer::new(
            Duration::from_secs_f32(1.0 * rand::random::<f32>() + 1.0),
            TimerMode::Once,
        );
        return;
    }
}

fn handle_submit(
    mut er: EventReader<Submit>,
    mut ew: EventWriter<Advance>,
    state: Res<State<CustomerState>>,
) {
    for _event in er.read() {
        match **state {
            CustomerState::Measuring => ew.send_default(),
            _ => {}
        }
    }
}
