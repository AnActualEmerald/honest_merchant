use bevy::{prelude::*, utils::HashMap};
use rand::prelude::*;
use serde::Deserialize;

use self::{
    customer::{CustomerPlugin, CustomerState},
    goods::GoodsPlugin,
    scales::ScalesPlugin,
};

mod customer;
mod goods;
mod scales;

pub use goods::ItemType;
pub use goods::ITEM_COST;
pub use scales::ScaleContents;

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TotalGold(f32);

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DailyGold(f32);

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DailyExpenses(f32);

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
        let mut iter = self.iter().peekable();

        while let Some((t, amount)) = iter.next() {
            if self.len() > 1 && iter.peek().is_none() {
                write!(f, "and ")?;
            }
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

            if iter.peek().is_some() {
                write!(f, ", ")?;
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

impl AttentionType {
    /// Returns the odds of becoming distracted and undistracted resepectively
    fn weights(&self) -> [(u32, u32); 2] {
        match self {
            Self::Alert => [(1, 100), (3, 4)],
            Self::Attentive => [(2, 50), (1, 2)],
            Self::Distracted => [(80, 90), (1, 500)],
            Self::Normal => [(1, 2), (1, 2)],
        }
    }

    /// Returns the percentage the amount has to be wrong for the customer to notice
    fn sus_threshold(&self) -> f32 {
        match self {
            Self::Alert => 0.1,
            Self::Attentive => 0.3,
            Self::Distracted => 0.7,
            Self::Normal => 0.5,
        }
    }
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
            .init_resource::<TotalGold>()
            .init_resource::<DailyGold>()
            .init_resource::<DailyExpenses>()
            .add_state::<GameState>()
            .add_event::<Advance>()
            .add_event::<UpdateScore>()
            .insert_resource(CustomerTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .insert_resource(DayTimer(Timer::from_seconds(60.0, TimerMode::Once)))
            .add_systems(
                Update,
                wait_for_customer.run_if(resource_exists::<CustomerTimer>()),
            )
            .add_systems(OnEnter(CustomerState::End), customer_end)
            .add_systems(
                Update,
                (
                    tick_day,
                    finish_day.run_if(in_state(GameState::Waiting)),
                ),
            );
    }
}

fn finish_day(
    timer: Res<DayTimer>,
    mut state: ResMut<NextState<GameState>>,
) {
    if timer.finished() {
        state.set(GameState::DayEnd);
    }
}

fn tick_day(mut timer: ResMut<DayTimer>, time: Res<Time>) {
    timer.tick(time.delta());
}

fn customer_end(
    mut timer: ResMut<CustomerTimer>,
    mut state: ResMut<NextState<GameState>>,
) {
    state.set(GameState::Waiting);
    let mut rng = SmallRng::from_entropy();
    *timer = CustomerTimer(Timer::from_seconds(
        rng.gen_range(3.0..=10.0),
        TimerMode::Once,
    ));
}

fn wait_for_customer(
    mut timer: ResMut<CustomerTimer>,
    time: Res<Time>,
    curr_state: Res<State<GameState>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if **curr_state == GameState::Waiting {
        timer.tick(time.delta());

        if timer.just_finished() {
            state.set(GameState::Customer);
        }
    }
}
