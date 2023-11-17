use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use serde::Deserialize;
use thiserror::Error;

use crate::game::{AttentionType, ItemRequest};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<CharacterTraits>()
            .init_asset_loader::<CharacteristicsLoader>();
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

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct CharacterTraits {
    pub name: String,
    pub greeting: Vec<String>,
    pub thinking: String,
    pub accept: String,
    pub reject: String,
    pub accuse: String,
    pub request: ItemRequest,
    pub attention_type: AttentionType,
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
            Ok(parsed)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["chr.ron", "chr", "char.ron", "char"]
    }
}
