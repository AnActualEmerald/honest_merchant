use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

use self::{
    customer::{CustomerPlugin, CustomerState},
    goods::{GoodsPlugin, ItemType},
    scales::ScalesPlugin,
};

mod customer;
mod goods;
mod scales;

#[derive(Event, Default, Debug, Clone, Copy)]
pub struct Advance;

#[derive(Resource, Debug, Clone, Deref, Default)]
pub struct TargetWeight(HashMap<ItemType, f32>);

impl From<ItemRequest> for TargetWeight {
    fn from(value: ItemRequest) -> Self {
        Self(value.0)
    }
}

impl From<&ItemRequest> for TargetWeight {
    fn from(value: &ItemRequest) -> Self {
       Self(value.0.clone())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemRequest(pub(crate) HashMap<ItemType, f32>);

impl std::ops::Deref for ItemRequest {
    type Target = HashMap<ItemType, f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ItemRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (t, amount) in self.iter() {
            match t {
                ItemType::Berries => {
                    write!(f, "{amount}g of berries")?;
                }
                ItemType::GreenMush => {
                    write!(f, "{amount}g of green mush")?;
                }
                ItemType::SpiderEyes => {
                    write!(f, "{amount}g of spider eyes")?;
                }
                ItemType::VibrantSyrup => {
                    write!(f, "{amount}g of vibrant syrup")?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum AttentionType {
    Distracted,
    Normal,
    Attentive,
    Alert,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, States, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Waiting,
    DayStart,
    DayEnd,
    Customer,
    Dialogue,
}

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct DayTimer(Timer);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct CustomerTimer(Timer);

#[derive(Event, Debug, Clone, Copy, Default)]
pub struct UpdateScore {
    pub rep: f32,
    pub gold: f32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CustomerPlugin, ScalesPlugin, GoodsPlugin))
            .add_state::<GameState>()
            .add_event::<Advance>()
            .add_event::<UpdateScore>()
            .insert_resource(CustomerTimer(Timer::from_seconds(
                5.0,
                TimerMode::Repeating,
            )))
            .add_systems(
                PreUpdate,
                wait_for_customer.run_if(
                    resource_exists::<CustomerTimer>().and_then(in_state(GameState::Waiting)),
                ),
            )
            .add_systems(
                Update,
                (customer_end).run_if(
                    in_state(GameState::Customer).and_then(state_changed::<CustomerState>()),
                ),
            );
    }
}

fn customer_end(mut state: ResMut<NextState<GameState>>, cust_state: Res<State<CustomerState>>) {
    if CustomerState::End == **cust_state {
        state.set(GameState::Waiting);
    }
}

fn wait_for_customer(mut cmd: Commands, mut timer: ResMut<CustomerTimer>, time: Res<Time>) {
    timer.tick(time.delta());

    if timer.just_finished() {
        cmd.insert_resource(NextState(Some(GameState::Customer)));
    }
}
