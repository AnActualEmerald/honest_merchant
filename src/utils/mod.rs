pub mod text_box;
use leafwing_input_manager::prelude::ActionState;
pub use text_box::spawn_text_box;

use bevy::prelude::*;

use crate::{game::Advance, input::Action};

use self::text_box::{TextBox, TextChild};

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, step_text).add_systems(PostStartup, initial_offset);
    }
}

// this is maybe too much for a single system to do?
fn step_text(
    mut box_q: Query<(&mut TextBox, &TextChild)>,
    mut text_q: Query<&mut Text>,
    actions: Res<ActionState<Action>>,
    mut ew: EventWriter<Advance>,
    time: Res<Time>,
) {
    for (mut b, child) in box_q.iter_mut() {
        if b.index <= b.text.len() {
            if b.timer.tick(time.delta()).just_finished() {
                let Ok(mut text) = text_q.get_mut(**child) else {
                    continue;
                };
                text.sections[0].value = b.text[..b.index].to_string();

                b.index += 1;
            } else {
                if actions.pressed(Action::Advance) {
                    // twice as fast text I think?
                    b.timer.tick(time.delta());
                }
            }
        } else {
            if actions.just_pressed(Action::Advance) {
                ew.send_default();
            }
        }
    }
}

fn initial_offset(mut q: Query<(&mut Transform, &Offset)>)  {
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
    fn is_about(self, target: Self, error: Self) -> bool;
}

impl Approx for f32 {
    fn is_about(self, target: Self, error: Self) -> bool {
        self <= target + error && self >= target - error
    }
}
