pub mod text_box;
pub use text_box::spawn_text_box;

use bevy::prelude::*;

use self::text_box::{TextBoxConfig, TextChild};

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, step_text);
    }
}

fn step_text(
    mut box_q: Query<(&mut TextBoxConfig, &TextChild)>,
    mut text_q: Query<&mut Text>,
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
                //TODO: listed for advance signal?
            }
        }
    }
}
