use crate::diagnostics::missing_file_finder::*;
use bevy::prelude::*;
pub struct SmartAssetIoPlugin;

impl Plugin for SmartAssetIoPlugin {
    fn build(&self, app: &mut App) {
        let default_io = AssetPlugin::default().create_platform_default_asset_io();

        // create the custom asset io instance
        let asset_io = SmartAssetIo(default_io);

        // the asset server is constructed and added the resource manager
        app.insert_resource(AssetServer::new(asset_io));
    }
}
