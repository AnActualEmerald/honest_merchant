use bevy::{prelude::*, utils::HashMap};
use rand::prelude::*;
use serde::Deserialize;

use crate::assets::CharacterTraits;
use crate::assets::Characters;

use self::{customer::CustomerPlugin, goods::GoodsPlugin, scales::ScalesPlugin};

mod customer;
mod goods;
mod scales;

pub use customer::CustomerState;
pub use goods::ItemType;
pub use goods::ITEM_COST;
pub use scales::ScaleContents;

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TotalGold(f32);

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DailyGold(f32);

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TotalExpenses(f32);

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
pub struct AttentionType {
    pub get_distracted: (u32, u32),
    pub get_focused: (u32, u32),
    pub threshold: f32,

}

impl AttentionType {
    /// Returns the odds of becoming distracted and undistracted resepectively
    fn weights(&self) -> [(u32, u32); 2] {
        [self.get_distracted, self.get_focused]
    }

    /// Returns the percentage the amount has to be wrong for the customer to notice
    fn sus_threshold(&self) -> f32 {
        self.threshold
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, States, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    Loading,
    Waiting,
    DayStart,
    DayEnd,
    Customer,
    Dialogue,
    GameOver,
}

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct DayTimer(Timer);

#[derive(Resource, Debug, Clone, Deref, DerefMut)]
pub struct CustomerTimer(Timer);

#[derive(Resource, Debug, Clone, Copy, Deref, DerefMut, Default)]
pub struct DayIndex(usize);

#[derive(Resource, Debug, Clone, Deref, DerefMut, Default)]
pub struct AvailableCustomers(Vec<Handle<CharacterTraits>>);

pub const DAY_LEN: f32 = 90.0;
pub const WEEK_LEN: usize = 5;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CustomerPlugin, ScalesPlugin, GoodsPlugin))
            .init_resource::<TotalGold>()
            .init_resource::<DailyGold>()
            .init_resource::<DailyExpenses>()
            .init_resource::<TotalExpenses>()
            .init_resource::<DayIndex>()
            .init_resource::<AvailableCustomers>()
            .add_state::<GameState>()
            .add_event::<Advance>()
            .insert_resource(CustomerTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .insert_resource(DayTimer(Timer::from_seconds(DAY_LEN, TimerMode::Once)))
            .add_systems(
                Update,
                wait_for_customer.run_if(
                    resource_exists::<CustomerTimer>().and_then(in_state(GameState::Waiting)),
                ),
            )
            .add_systems(OnEnter(GameState::DayEnd), accounting)
            .add_systems(OnEnter(GameState::GameOver), accounting)
            .add_systems(
                OnEnter(CustomerState::End),
                customer_end.run_if(in_state(GameState::Customer)),
            )
            .add_systems(
                Update,
                (tick_day, finish_day.run_if(in_state(GameState::Waiting))),
            )
            .add_systems(OnEnter(GameState::DayStart), (start_day, customer_end));
    }
}

fn accounting(mut total_g: ResMut<TotalGold>, mut total_e: ResMut<TotalExpenses>, daily_g: Res<DailyGold>, daily_e: Res<DailyExpenses>) {
    **total_g += **daily_g;
    **total_e += **daily_e;
}

fn start_day(
    mut gold: ResMut<DailyGold>,
    mut expenses: ResMut<DailyExpenses>,
    mut state: ResMut<NextState<GameState>>,
    mut timer: ResMut<DayTimer>,
    mut available: ResMut<AvailableCustomers>,
    day: Res<DayIndex>,
    customers: Res<Characters>,
) {
    **gold = 0.0;
    **expenses = 0.0;
    timer.reset();

    *available = match **day {
        0 => AvailableCustomers(vec![customers.dumb.clone()]),
        1 => AvailableCustomers(vec![customers.dumb.clone(), customers.normal.clone()]),
        2 => AvailableCustomers(vec![customers.normal.clone(), customers.attentive.clone()]),
        3 => AvailableCustomers(vec![
            customers.normal.clone(),
            customers.attentive.clone(),
            customers.cop.clone(),
        ]),
        4 => AvailableCustomers(vec![customers.attentive.clone(), customers.cop.clone()]),
        _ => AvailableCustomers::default(),
    };

    state.set(GameState::Waiting);
}

fn finish_day(
    timer: Res<DayTimer>,
    mut state: ResMut<NextState<GameState>>,
    mut day: ResMut<DayIndex>,
) {
    if timer.finished() {
        **day += 1;
        if **day >= WEEK_LEN {
            state.set(GameState::GameOver);
        } else {
            state.set(GameState::DayEnd);
        }
    }
}

fn tick_day(mut timer: ResMut<DayTimer>, time: Res<Time>) {
    timer.tick(time.delta());
}

fn customer_end(mut timer: ResMut<CustomerTimer>, mut state: ResMut<NextState<GameState>>) {
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
