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
        CalcCost, PercentDiff, Ratios, Total,
    },
};

use super::{
    scales::{self, ScaleContents, ScaleWeights, Submit, SusEvent, ScaleIsSus},
    Advance, AvailableCustomers, DailyExpenses, DailyGold, GameState, TargetWeight,
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

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AttentionState {
    #[default]
    Attent = 0,
    Distracted,
}

impl AttentionState {
    pub fn other(&self) -> Self {
        match self {
            Self::Attent => Self::Distracted,
            Self::Distracted => Self::Attent,
        }
    }
}



#[derive(Component, Clone, Copy, Debug)]
#[component(storage = "SparseSet")]
pub struct WillChange;

#[derive(Component)]
pub struct Customer(Handle<CharacterTraits>);

pub const CUSTOMER_STAND_POINT: Vec3 = Vec3::new(0.0, 0.0, -3.0);

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CustomerState>()
            .add_state::<AttentionState>()
            .init_resource::<TargetWeight>()
            .add_systems(
                Update,
                (
                    handle_submit,
                    wait_to_advance,
                    get_distracted,
                    (attention_gizmos, handle_attention).run_if(in_state(AttentionState::Attent)),
                ),
            )
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

fn attention_gizmos(mut gizmos: Gizmos) {
    let trans = Transform::from_translation(CUSTOMER_STAND_POINT + Vec3::new(0.0, 2.0, 0.25));
    gizmos.cuboid(trans, Color::GREEN);
}

// TODO: This should animate in a customer from the crowd
fn spawn_customer(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<NextState<CustomerState>>,
    available: Res<AvailableCustomers>,
) {
    let mut rng = SmallRng::from_entropy();
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
        Customer(
            available
                .choose(&mut rng)
                .expect("No available customer types")
                .clone(),
        ),
    ));

    state.set(CustomerState::Greeting);
}

fn get_distracted(
    mut cmd: Commands,
    q: Query<(Entity, &Customer)>,
    chars: Res<Assets<CharacterTraits>>,
    time: Res<Time>,
    current_state: Res<State<AttentionState>>,
    mut state: ResMut<NextState<AttentionState>>,
    mut will_change: Local<bool>,
    mut delay: Local<Timer>,
    mut lockout: Local<Timer>,
) {
    for (ent, cust) in q.iter() {
        // ensure a minimum amount of time spent in a state
        // kind of debouncing I guess
        if !lockout.finished() {
            lockout.tick(time.delta());
            continue;
        }
        if !*will_change {
            let mut rng = SmallRng::from_entropy();
            let Some(traits) = chars.get(&cust.0) else {
                continue;
            };
            let weights = traits.attention_type.weights()[*current_state.get() as usize];
            if rng.gen_ratio(weights.0, weights.1) {
                // add a component before actually changing to enable giving some kind of
                // visual cue to the player
                *will_change = true;
                *delay = Timer::new(Duration::from_millis(500), TimerMode::Once);
                cmd.entity(ent).insert(WillChange);
            }
        } else if delay.tick(time.delta()).just_finished() {
            state.set(current_state.get().other());
            *will_change = false;
            cmd.entity(ent).remove::<WillChange>();
            *lockout = Timer::new(Duration::from_millis(1000), TimerMode::Once);
        }
    }
}

fn handle_attention(
    mut state: ResMut<NextState<CustomerState>>,
    mut events: EventReader<SusEvent>,

) {
    for _e in events.read() {
        state.set(CustomerState::Angry);
    }
}

fn cleanup(mut tw: ResMut<TargetWeight>) {
    *tw = TargetWeight::default();
}

fn show_text(
    cust_q: Query<&Customer>,
    state: Res<State<CustomerState>>,
    chars: Res<Assets<CharacterTraits>>,
    mut target: ResMut<TargetWeight>,
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
                let mut rng = SmallRng::from_entropy();
                let req = ty.request.choose(&mut rng).expect("No item requests");
                let req_text = format!("{} please", req);
                spawn_text.send(req_text.into());
                *target = TargetWeight::from(req);
            }
            CustomerState::Review => {
                spawn_text.send(ty.thinking.clone().into());
            }
            CustomerState::Payment => {
                spawn_text
                    .send(format!("{} Here's {} gold", ty.accept, target.customer_cost()).into());
            }
            CustomerState::Reject => {
                spawn_text.send(ty.reject.clone().into());
            }
            CustomerState::Angry => spawn_text.send(ty.accuse.clone().into()),
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

fn pay(
    mut gold: ResMut<DailyGold>,
    mut expenses: ResMut<DailyExpenses>,
    target: Res<TargetWeight>,
    contents: Res<ScaleContents>,
) {
    **gold += target.customer_cost();
    **expenses += contents.cost();
}

fn handle_review(
    is_sus: Option<Res<ScaleIsSus>>,
    scale_weights: Res<ScaleWeights>,
    contents: Res<ScaleContents>,
    target: Res<TargetWeight>,
    q: Query<&Customer>,
    chars: Res<Assets<CharacterTraits>>,
    mut timer: Local<Timer>,
    time: Res<Time>,
    mut state: ResMut<NextState<CustomerState>>,
) {
    if !timer.finished() {
        timer.tick(time.delta());

        if timer.just_finished() {
            info!("{contents:?} vs {target:?}");
            info!("Diff: {}", target.diff(&**contents));
            let cust = q.get_single().expect("No customer?");
            let traits = chars.get(&cust.0).expect("Unable to get traits");
            // customers can tell when the amount isn't correct
            if traits.attention_type.sus_threshold() < target.diff(&**contents)
            // don't let customers be fooled without using the sus weights
                || ((target.total() != contents.total()) && is_sus.is_none())
            {
                state.set(CustomerState::Angry);
            } else if scale_weights.is_even() && target.ratio() == contents.ratio() {
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
