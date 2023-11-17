pub mod text_box;
use leafwing_input_manager::prelude::ActionState;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    game::{Advance, ItemType, ITEM_COST},
    input::Action,
};

use self::text_box::{spawn_text_box, SpawnTextBox, TimedText};

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTextBox>()
            .add_systems(Update, (spawn_text_box, step_text.after(spawn_text_box)))
            .add_systems(PostStartup, initial_offset);
    }
}

fn step_text(
    mut text_q: Query<(&mut Text, &mut TimedText)>,
    actions: Res<ActionState<Action>>,
    mut ew: EventWriter<Advance>,
    time: Res<Time>,
) {
    for (mut text, mut timed) in text_q.iter_mut() {
        info!("Text step");
        if timed.index <= timed.text.len() {
            if timed.timer.tick(time.delta()).just_finished() {
                text.sections[0].value = timed.text[..timed.index].to_string();

                timed.index += 1;
            } else if actions.pressed(Action::Advance) {
                // twice as fast text I think?
                timed.timer.tick(time.delta());
            }
        } else if actions.just_pressed(Action::Advance) {
            ew.send_default();
        }
    }
}

fn initial_offset(mut q: Query<(&mut Transform, &Offset)>) {
    for (mut tr, off) in q.iter_mut() {
        tr.translation += **off;
    }
}

pub fn despawn_all<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for ent in q.iter() {
        cmd.entity(ent).despawn_recursive();
    }
}

#[derive(Component, Deref, DerefMut, Clone, Debug, Default)]
pub struct Offset(Vec3);

impl Offset {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3 { x, y, z })
    }
}

pub trait RoundTo {
    fn round_to(self, precision: i32) -> Self;
}

impl RoundTo for f32 {
    fn round_to(self, precision: i32) -> Self {
        (self * (10 as Self).powi(precision)).round() / (10.0 as Self).powi(precision)
    }
}

pub trait Approx {
    fn is_about(&self, target: Self, error: Self) -> bool;
}

impl Approx for f32 {
    fn is_about(&self, target: Self, error: Self) -> bool {
        *self <= target + error && *self >= target - error
    }
}

pub trait CalcCost {
    fn cost(&self) -> f32;
    fn customer_cost(&self) -> f32 {
        self.cost() * 2.0
    }
}

impl CalcCost for HashMap<ItemType, f32> {
    fn cost(&self) -> f32 {
        self.iter()
            .map(|(t, amnt)| amnt * ITEM_COST[*t as usize])
            .sum::<f32>()
    }
}

pub trait Ratios {
    type Output;
    fn ratio(&self) -> Self::Output;
}

impl Ratios for HashMap<ItemType, f32> {
    type Output = HashMap<ItemType, f32>;

    fn ratio(&self) -> Self::Output {
        let total: f32 = self.values().sum();
        self.iter()
            .map(|(k, v)| {
                let r = v / total;
                (*k, r)
            })
            .collect()
    }
}
