use bevy::prelude::*;

#[derive(Component)]
pub struct TextBoxConfig {
    pub timer: Timer,
    pub index: usize,
    pub text: String,
}

#[derive(Component, Deref)]
pub struct TextChild(Entity);

pub fn spawn_text_box(cmd: &mut Commands, text: impl Into<String>) -> Entity {
    let root = cmd.spawn_empty().id();
    cmd.entity(root).insert((
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
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
        TextBoxConfig {
            timer: Timer::from_seconds(0.01, TimerMode::Repeating),
            index: 0,
            text: text.into()
        },
    ));
    let text = cmd
        .spawn(TextBundle::from_section("", TextStyle::default()))
        .id();

    cmd.entity(root).add_child(text).insert(TextChild(text));

    root
}
