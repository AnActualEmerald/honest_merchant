use bevy::{audio::VolumeLevel, prelude::*};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{
    assets::Sounds,
    game::{AddItem, AddWeight, GameState, ItemType, RemoveItem, RemoveWeight, Submit},
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), bg_music)
            .add_systems(
                Update,
                (
                    sfx_add_weight.run_if(on_event::<AddWeight>()),
                    sfx_remove_weight.run_if(on_event::<RemoveWeight>()),
                    sfx_add_item.run_if(resource_exists::<Sounds>()),
                    sfx_remove_item.run_if(on_event::<RemoveItem>()),
                    sfx_submit.run_if(on_event::<Submit>()),
                ),
            );
    }
}

fn bg_music(mut cmd: Commands, sounds: Res<Sounds>) {
    cmd.spawn(AudioBundle {
        source: sounds.music.clone(),
        settings: PlaybackSettings::LOOP
            .with_volume(bevy::audio::Volume::Relative(VolumeLevel::new(0.1))),
        ..default()
    });
}

fn sfx_add_weight(mut cmd: Commands, sounds: Res<Sounds>) {
    cmd.spawn(AudioBundle {
        source: sounds.pop1.clone(),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

fn sfx_remove_weight(mut cmd: Commands, sounds: Res<Sounds>) {
    cmd.spawn(AudioBundle {
        source: sounds.pop2.clone(),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

fn sfx_add_item(
    mut cmd: Commands,
    sounds: Res<Sounds>,
    mut events: EventReader<AddItem>,
    type_q: Query<&ItemType>,
) {
    let mut rng = SmallRng::from_entropy();
    for event in events.read() {
        let Ok(t) = type_q.get(**event) else {
            continue;
        };

        match *t {
            ItemType::GreenMush | ItemType::VibrantSyrup | ItemType::Berries => {
                cmd.spawn(AudioBundle {
                    source: sounds.scoop_wet.clone(),
                    settings: PlaybackSettings::DESPAWN.with_speed(rng.gen_range(0.75..=1.25)),
                    ..default()
                });
            }
            ItemType::SpiderEyes => {
                cmd.spawn(AudioBundle {
                    source: sounds.scoop_hard.clone(),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            }
        }
    }
}

fn sfx_remove_item(mut cmd: Commands, sounds: Res<Sounds>) {
    cmd.spawn(AudioBundle {
        source: sounds.trash.clone(),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

fn sfx_submit(mut cmd: Commands, sounds: Res<Sounds>) {
    cmd.spawn(AudioBundle {
        source: sounds.bell.clone(),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}
