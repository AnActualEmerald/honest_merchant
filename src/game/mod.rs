use bevy::prelude::*;
use serde::Deserialize;

use self::{customer::CustomerPlugin, scales::ScalesPlugin};

mod customer;
mod scales;

#[derive(Event, Default, Debug, Clone, Copy)]
pub struct Advance;

#[derive(Resource, Debug, Clone, Copy, Deref)]
pub struct TargetWeight(f32);

impl From<ItemType> for TargetWeight {
    fn from(value: ItemType) -> Self {
        match value {
            ItemType::Berries(v) => Self(v),
            ItemType::GreenMush(v) => Self(v),
            ItemType::SpiderEyes(v) => Self(v),
            ItemType::VibrantSyrup(v) => Self(v),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum AttentionType {
    Distracted,
    Normal,
    Attentive,
    Alert,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[serde(tag = "kind", content = "amount")]
pub enum ItemType {
    Berries(f32),
    GreenMush(f32),
    SpiderEyes(f32),
    VibrantSyrup(f32),
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Berries(amount) => {
                write!(f, "{amount}g of berries")
            }
            Self::GreenMush(amount) => {
                write!(f, "{amount}g of green mush")
            }
            Self::SpiderEyes(amount) => {
                write!(f, "{amount}g of spider eyes")
            }
            Self::VibrantSyrup(amount) => {
                write!(f, "{amount}g of vibrant syrup")
            }
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CustomerPlugin, ScalesPlugin))
            .add_state::<GameState>()
            .add_event::<Advance>()
            .insert_resource(CustomerTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .add_systems(
                PreUpdate,
                wait_for_customer.run_if(resource_exists::<CustomerTimer>()),
            );
    }
}

fn wait_for_customer(mut cmd: Commands, mut timer: ResMut<CustomerTimer>, time: Res<Time>) {
    timer.tick(time.delta());

    if timer.just_finished() {
        cmd.insert_resource(NextState(Some(GameState::Customer)));
    }
}
