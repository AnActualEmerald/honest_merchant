use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;
use thiserror::Error;

use crate::{
    game::{AttentionType, GameState, ItemRequest},
    AppState,
};

#[derive(AssetCollection, Resource)]
pub struct Meshes {
    #[asset(path = "stand.glb")]
    pub stand: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct Images {
    #[asset(path = "example.png")]
    pub example: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
    #[asset(path = "fonts/Inconsolata-Medium.ttf")]
    pub default: Handle<Font>,
    #[asset(path = "fonts/Lugrasimo-Regular.ttf")]
    pub handwritten: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct Characters {
    #[asset(path = "customers/dumb.chr.ron")]
    pub dumb: Handle<CharacterTraits>,
    #[asset(path = "customers/attentive.chr.ron")]
    pub attentive: Handle<CharacterTraits>,
    #[asset(path = "customers/normal.chr.ron")]
    pub normal: Handle<CharacterTraits>,
    #[asset(path = "customers/cop.chr.ron")]
    pub cop: Handle<CharacterTraits>,
}

#[derive(AssetCollection, Resource)]
pub struct Splash {
    #[asset(path = "fonts/Inconsolata-Medium.ttf")]
    pub font: Handle<Font>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharacterTraits>()
            .init_asset_loader::<CharacteristicsLoader>()
            .add_loading_state(LoadingState::new(AppState::Load).continue_to_state(AppState::Done))
            .add_collection_to_loading_state::<_, Splash>(AppState::Load)
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
            )
            .add_collection_to_loading_state::<_, Fonts>(GameState::Loading)
            .add_collection_to_loading_state::<_, Meshes>(GameState::Loading)
            .add_collection_to_loading_state::<_, Images>(GameState::Loading)
            .add_collection_to_loading_state::<_, Characters>(GameState::Loading);
    }
}

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("Error reading bytes: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error parsing RON: {0}")]
    RonError(#[from] ron::error::SpannedError),
}

// character files

#[derive(Asset, TypePath, Debug, Deserialize, Clone)]
pub struct CharacterTraits {
    pub name: String,
    pub color: Color,
    pub greeting: Vec<String>,
    pub thinking: String,
    pub accept: String,
    pub reject: String,
    pub accuse: String,
    pub request: Vec<ItemRequest>,
    pub attention_type: AttentionType,
    pub rep_hit: u8,
}

#[derive(Default)]
pub struct CharacteristicsLoader;

impl AssetLoader for CharacteristicsLoader {
    type Asset = CharacterTraits;

    type Settings = ();

    type Error = LoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _: &'a Self::Settings,
        _: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut raw = Vec::new();
            reader.read_to_end(&mut raw).await?;
            let parsed = ron::de::from_bytes(&raw)?;
            info!("{parsed:?}");
            Ok(parsed)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["chr.ron", "chr", "char.ron", "char"]
    }
}
