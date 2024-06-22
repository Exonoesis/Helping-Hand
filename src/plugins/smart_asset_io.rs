use crate::diagnostics::missing_file_finder::*;

use bevy::prelude::*;
use bevy::asset::io::{AssetSourceId, AssetSource};

pub struct SmartAssetReaderPlugin;

impl Plugin for SmartAssetReaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(|| {
                Box::new(SmartAssetReader(
                    AssetSource::get_default_reader("assets".to_string())(),
                ))
            }),
        );
    }
}
