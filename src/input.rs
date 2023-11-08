pub use bevy::prelude::*;

use crate::WINDOW_SIZE;

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct CursorPos(Vec2);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_systems(PreUpdate, update_cursor_pos);
    }
}

fn update_cursor_pos(mut pos: ResMut<CursorPos>, mut events: EventReader<CursorMoved>) {
    for event in events.read() {
        // this shifts the origin for the cursor to match the coordinate system
        *pos = CursorPos((event.position - (WINDOW_SIZE / 2.0)) * Vec2::new(1.0, -1.0));
    }
}
