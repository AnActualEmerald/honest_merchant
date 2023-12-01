use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotateXLens},
    *,
};
use leafwing_input_manager::action_state::ActionState;
use serde::Deserialize;
use strum::{EnumCount, FromRepr};

use crate::{input::Action, ui::tooltips::TooltipBundle, utils::Offset};

use super::{scales::ScaleContents, GameState};

#[derive(
    Component,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumCount,
    FromRepr,
)]
pub enum ItemType {
    Berries = 0,
    GreenMush,
    SpiderEyes,
    VibrantSyrup,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Berries => write!(f, "berries"),
            Self::GreenMush => write!(f, "green mush"),
            Self::SpiderEyes => write!(f, "spider eyes"),
            Self::VibrantSyrup => write!(f, "vibrant syrup"),
        }
    }
}

#[derive(Event, Deref)]
pub struct AddItem(Entity);

impl From<ListenerInput<Pointer<Down>>> for AddItem {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self(value.target)
    }
}

#[derive(Event, Deref)]
pub struct RemoveItem(Entity);

impl From<ListenerInput<Pointer<Down>>> for RemoveItem {
    fn from(value: ListenerInput<Pointer<Down>>) -> Self {
        Self(value.target)
    }
}

#[derive(Event, Clone, Copy)]
pub struct Open(Entity);

impl From<ListenerInput<Pointer<Over>>> for Open {
    fn from(value: ListenerInput<Pointer<Over>>) -> Self {
        Self(value.target)
    }
}

#[derive(Event, Clone, Copy)]
pub struct Close(Entity);

impl From<ListenerInput<Pointer<Out>>> for Close {
    fn from(value: ListenerInput<Pointer<Out>>) -> Self {
        Self(value.target)
    }
}

pub const BOX_WIDTH: f32 = 0.25;
pub const BOX_HEIGHT: f32 = 0.5;
pub const ANIM_DURATION: u64 = 50;

pub const DRAWER_CLOSED_POS: Transform = Transform::from_xyz(0.5 - 0.3, 0.5, -0.2);
pub const DRAWER_OPEN_POS: Transform = Transform::from_translation(Vec3 {
    x: DRAWER_CLOSED_POS.translation.x,
    y: DRAWER_CLOSED_POS.translation.y,
    z: DRAWER_CLOSED_POS.translation.z + 0.125,
}); // adding vectors isn't const

pub const ITEM_COLORS: [Color; ItemType::COUNT] = [
    Color::CRIMSON,
    Color::DARK_GREEN,
    Color::SEA_GREEN,
    Color::rgb(1.0, 0.0, 1.0),
];

/// How much each item costs per 1 g
pub const ITEM_COST: [f32; ItemType::COUNT] = [1.0, 2.0, 5.0, 8.0];

pub struct GoodsPlugin;

impl Plugin for GoodsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddItem>()
            .add_event::<RemoveItem>()
            .add_event::<Open>()
            .add_event::<Close>()
            .add_systems(OnEnter(GameState::MainMenu), spawn_goods)
            .add_systems(
                Update,
                (
                    animate_drawers_open,
                    animate_drawers_close,
                    handle_add,
                    handle_remove,
                ),
            );
    }
}

fn spawn_goods(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_mesh = meshes.add(shape::Box::new(BOX_WIDTH, BOX_HEIGHT, 0.5).into());

    cmd.spawn((
        PbrBundle {
            mesh: box_mesh.clone(),
            material: materials.add(ITEM_COLORS[ItemType::SpiderEyes as usize].into()),
            transform: DRAWER_CLOSED_POS,
            ..default()
        },
        ItemType::SpiderEyes,
        On::<Pointer<Down>>::send_event::<AddItem>(),
        On::<Pointer<Over>>::send_event::<Open>(),
        On::<Pointer<Out>>::send_event::<Close>(),
        Offset::default(),
        TooltipBundle::new("Spider Eyes"),
    ));

    cmd.spawn((
        PbrBundle {
            mesh: box_mesh.clone(),
            material: materials.add(ITEM_COLORS[ItemType::Berries as usize].into()),
            transform: DRAWER_CLOSED_POS,
            ..default()
        },
        ItemType::Berries,
        On::<Pointer<Down>>::send_event::<AddItem>(),
        On::<Pointer<Over>>::send_event::<Open>(),
        On::<Pointer<Out>>::send_event::<Close>(),
        Offset::new(0.3, 0.0, 0.0),
        TooltipBundle::new("Berries"),
    ));

    cmd.spawn((
        PbrBundle {
            mesh: box_mesh.clone(),
            material: materials.add(ITEM_COLORS[ItemType::GreenMush as usize].into()),
            transform: DRAWER_CLOSED_POS,
            ..default()
        },
        ItemType::GreenMush,
        On::<Pointer<Down>>::send_event::<AddItem>(),
        On::<Pointer<Over>>::send_event::<Open>(),
        On::<Pointer<Out>>::send_event::<Close>(),
        Offset::new(0.6, 0.0, 0.0),
        TooltipBundle::new("Green Mush"),
    ));
    cmd.spawn((
        PbrBundle {
            mesh: box_mesh.clone(),
            material: materials.add(ITEM_COLORS[ItemType::VibrantSyrup as usize].into()),
            transform: DRAWER_CLOSED_POS,
            ..default()
        },
        ItemType::VibrantSyrup,
        On::<Pointer<Down>>::send_event::<AddItem>(),
        On::<Pointer<Over>>::send_event::<Open>(),
        On::<Pointer<Out>>::send_event::<Close>(),
        Offset::new(0.9, 0.0, 0.0),
        TooltipBundle::new("Vibrant Syrup"),
    ));
}

fn handle_add(
    mut er: EventReader<AddItem>,
    q: Query<&ItemType>,
    mut contents: ResMut<ScaleContents>,
    actions: Res<ActionState<Action>>,
) {
    for event in er.read() {
        let Ok(t) = q.get(event.0) else { continue };

        let amnt = if actions.pressed(Action::Mod) {
            0.5
        } else {
            1.0
        };

        if let Some(val) = contents.get_mut(t) {
            *val += amnt;
        } else {
            contents.insert(*t, amnt);
        }
    }
}

fn handle_remove(
    mut er: EventReader<RemoveItem>,
    q: Query<&ItemType>,
    mut contents: ResMut<ScaleContents>,
    actions: Res<ActionState<Action>>,
) {
    for event in er.read() {
        let Ok(t) = q.get(event.0) else { continue };

        let amnt = if actions.pressed(Action::Mod) {
            0.5
        } else {
            1.0
        };

        if let Some(val) = contents.get_mut(t) {
            *val -= amnt;
            // *val = val.max(0.0);
        } else {
            continue;
        }

        contents.retain(|_, v| *v > 0.0);
    }
}

fn animate_drawers_open(
    mut cmd: Commands,
    mut er: EventReader<Open>,
    q: Query<(Entity, &Offset), With<ItemType>>,
) {
    for event in er.read() {
        let Ok((ent, off)) = q.get(event.0) else {
            continue;
        };
        let slide_out = Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(ANIM_DURATION),
            TransformPositionLens {
                start: DRAWER_CLOSED_POS.translation + **off,
                end: DRAWER_OPEN_POS.translation + **off,
            },
        )
        .then(Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(ANIM_DURATION),
            TransformRotateXLens {
                start: 0.0,
                end: 30.0f32.to_radians(),
            },
        ));

        cmd.entity(ent).insert(Animator::new(slide_out));
    }
}

fn animate_drawers_close(
    mut cmd: Commands,
    mut er: EventReader<Close>,
    q: Query<(Entity, &Offset), With<ItemType>>,
) {
    for event in er.read() {
        let Ok((ent, off)) = q.get(event.0) else {
            continue;
        };
        let slide_in = Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(ANIM_DURATION),
            TransformRotateXLens {
                end: 0.0,
                start: 30.0f32.to_radians(),
            },
        )
        .then(Tween::new(
            EaseMethod::Linear,
            Duration::from_millis(ANIM_DURATION),
            TransformPositionLens {
                end: DRAWER_CLOSED_POS.translation + **off,
                start: DRAWER_OPEN_POS.translation + **off,
            },
        ));

        cmd.entity(ent).insert(Animator::new(slide_in));
    }
}
