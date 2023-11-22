use std::time::Duration;

use bevy::prelude::*;

use crate::assets::Fonts;

#[derive(Component, Debug, Clone)]
pub struct TimedText {
    pub timer: Timer,
    pub index: usize,
    pub text: String,
}

#[derive(Component, Debug)]
pub struct TextBox;

#[derive(Component, Deref)]
pub struct TextChild(Entity);

#[derive(Event, Debug, Clone, Deref)]
pub struct SpawnTextBox(pub Vec<String>);

impl<T: Into<String>> From<T> for SpawnTextBox {
    fn from(value: T) -> Self {
        Self(vec![value.into()])
    }
}

pub fn spawn_text_box(
    mut cmd: Commands,
    fonts: Res<Fonts>,
    mut events: EventReader<SpawnTextBox>,
) {
    for e in events.read() {
        cmd.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(75.0),
                    height: Val::Percent(30.0),
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::FlexStart,
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Auto,
                    margin: UiRect::all(Val::Px(20.0)),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            TextBox,
        ))
        .with_children(|parent| {
            for t in e.iter() {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: fonts.default.clone(),
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    TimedText {
                        timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
                        index: 0,
                        text: t.clone()
                    },
                ));
            }
        });
    }
}
