use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::assets::Fonts;

use super::NeedsTextSet;

#[derive(Event)]
pub struct ShowTooltip(Entity, Vec2);

impl From<ListenerInput<Pointer<Over>>> for ShowTooltip {
    fn from(value: ListenerInput<Pointer<Over>>) -> Self {
        Self(value.target, value.pointer_location.position)
    }
}

#[derive(Event)]
pub struct RemoveTooltip(Entity);

impl From<ListenerInput<Pointer<Out>>> for RemoveTooltip {
    fn from(value: ListenerInput<Pointer<Out>>) -> Self {
        Self(value.target)
    }
}

#[derive(Bundle)]
pub struct TooltipBundle {
    pub text: TooltipText,
    // pub over_listener: On<Pointer<Over>>,
    // pub out_listener: On<Pointer<Out>>
}

impl TooltipBundle {
    pub fn new(text: impl Into<TooltipText>) -> Self {
        Self {
            text: text.into(),
            // over_listener: On::<Pointer<Over>>::send_event::<ShowTooltip>(),
            // out_listener: On::<Pointer<Out>>::send_event::<RemoveTooltip>(),
        }
    }
}

#[derive(Component, Debug)]
pub struct TooltipText(pub String);

impl<T: Into<String>> From<T> for TooltipText {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Component, Debug)]
pub struct Tooltip {
    target: Entity,
}

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShowTooltip>()
            .add_event::<RemoveTooltip>()
            .add_systems(PreUpdate, update_tooltips.in_set(NeedsTextSet))
            .add_systems(Update, (show_tooltips, despawn_tooltips).in_set(NeedsTextSet));
    }
}

fn update_tooltips(
    mut cmd: Commands,
    mut tt_q: Query<(Entity, &mut Style, &mut Visibility, &mut Text, &Node, &Tooltip)>,
    text_q: Query<&TooltipText>,
    window_q: Query<&Window>,
    mut er: EventReader<CursorMoved>,
) {
    for event in er.read() {
        for (ent, mut style, mut vis, mut text, node, tt) in tt_q.iter_mut() {
            if let Ok(updated_text) = text_q.get(tt.target) {
                let Ok(window) = window_q.get_single() else {
                    error!("Getting single window failed");
                    continue;
                };

                let size = node.size();
                let mut x = event.position.x;
                let mut y = event.position.y - size.y;

                if x + size.x > window.width() {
                    x -= size.x;
                }

                if y < 0.0 {
                    y += size.y;
                }


                style.left = Val::Px(x);
                style.top = Val::Px(y);

                text.sections[0].value = updated_text.0.clone();

                *vis = Visibility::Visible;
            } else {
                cmd.entity(ent).despawn_recursive();
            }
        }
    }
}

fn despawn_tooltips(mut cmd: Commands, mut er: EventReader<Pointer<Out>>, q: Query<(Entity, &Tooltip)>) {
    for event in er.read() {
        if let Some((ent, _)) = q.iter().find(|(_, tt)| tt.target == event.target) {
            cmd.entity(ent).despawn_recursive();
        }
    }
}

fn show_tooltips(
    mut er: EventReader<Pointer<Over>>,
    mut cmd: Commands,
    q: Query<&TooltipText>,
    fonts: Res<Fonts>,
) {
    for event in er.read() {
        if let Ok(text) = q.get(event.target) {
            cmd.spawn((
                TextBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(10.0)),
                        top: Val::Px(event.pointer_location.position.y),
                        left: Val::Px(event.pointer_location.position.x),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    background_color: Color::WHITE.into(),
                    text: Text::from_section(
                        text.0.clone(),
                        TextStyle {
                            font: fonts.default.clone(),
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    ..default()
                },
                Tooltip { target: event.target },
                Pickable::IGNORE,
            ));
        }
    }
}
