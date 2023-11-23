pub use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    user_input::InputKind,
    Actionlike,
};

use crate::WINDOW_SIZE;

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct CursorPos(Vec2);

#[derive(Actionlike, TypePath, Clone, Copy)]
pub enum Action {
    Advance,
    Mod,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .init_resource::<ActionState<Action>>()
            .insert_resource(InputMap::new([
                (InputKind::Keyboard(KeyCode::Space), Action::Advance),
                (InputKind::Mouse(MouseButton::Left), Action::Advance),
                (InputKind::Keyboard(KeyCode::ShiftLeft), Action::Mod),
            ]))
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_systems(PreUpdate, update_cursor_pos);
    }
}

fn update_cursor_pos(mut pos: ResMut<CursorPos>, mut events: EventReader<CursorMoved>) {
    for event in events.read() {
        // this shifts the origin for the cursor to match the coordinate system
        *pos = CursorPos((event.position - (WINDOW_SIZE / 2.0)) * Vec2::new(1.0, -1.0));
    }
}
