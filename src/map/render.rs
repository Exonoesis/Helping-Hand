use std::path::{Path, PathBuf};

use bevy::{asset::UntypedAssetId, prelude::*};

use crate::{
    map::{
        movement::grid_based_movement::MovementDirection, npc::NPC, player::Player, GridCords3D,
    },
    narrative::act_loading::LoadStatus,
};

use super::{flip_y_axis, PxDimensions, Tile};

#[derive(Bundle, Clone)]
pub struct SpriteBundle {
    sprite: Sprite,
    visibility: Visibility,
    transform: Transform,
    load_status: LoadStatus,
}

impl SpriteBundle {
    pub fn new(
        sprite: Sprite,
        visibility: Visibility,
        transform: Transform,
        load_status: LoadStatus,
    ) -> Self {
        Self {
            sprite,
            visibility,
            transform,
            load_status,
        }
    }
}

pub struct Environment;

#[derive(Bundle)]
pub struct EnvironmentalTile {
    tile_dimensions: PxDimensions,
    grid_coordinate: GridCords3D,
    sprite_bundle: SpriteBundle,
}

impl EnvironmentalTile {
    pub fn new(
        tile_dimensions: PxDimensions,
        grid_coordinate: GridCords3D,
        sprite_bundle: SpriteBundle,
    ) -> Self {
        Self {
            tile_dimensions,
            grid_coordinate,
            sprite_bundle,
        }
    }

    pub fn get_tile_dimensions(&self) -> &PxDimensions {
        &self.tile_dimensions
    }

    pub fn get_grid_coordinates(&self) -> &GridCords3D {
        &self.grid_coordinate
    }
}

#[derive(Bundle)]
pub struct PlayerTile {
    tile_dimensions: PxDimensions,
    grid_coordinate: GridCords3D,
    sprite_bundle: SpriteBundle,
    player: Player,
    movement_direction: MovementDirection,
}

impl PlayerTile {
    pub fn new(
        tile_dimensions: PxDimensions,
        grid_coordinate: GridCords3D,
        sprite_bundle: SpriteBundle,
        player: Player,
        movement_direction: MovementDirection,
    ) -> Self {
        Self {
            tile_dimensions,
            grid_coordinate,
            sprite_bundle,
            player,
            movement_direction,
        }
    }

    pub fn get_tile_dimensions(&self) -> &PxDimensions {
        &self.tile_dimensions
    }

    pub fn get_grid_coordinates(&self) -> &GridCords3D {
        &self.grid_coordinate
    }

    pub fn get_name(&self) -> &String {
        &self.player.get_name()
    }
}

#[derive(Bundle)]
pub struct NPCTile {
    tile_dimensions: PxDimensions,
    grid_coordinate: GridCords3D,
    sprite_bundle: SpriteBundle,
    npc: NPC,
}

impl NPCTile {
    pub fn new(
        tile_dimensions: PxDimensions,
        grid_coordinate: GridCords3D,
        sprite_bundle: SpriteBundle,
        npc: NPC,
    ) -> Self {
        Self {
            tile_dimensions,
            grid_coordinate,
            sprite_bundle,
            npc,
        }
    }

    pub fn get_tile_dimensions(&self) -> &PxDimensions {
        &self.tile_dimensions
    }

    pub fn get_grid_coordinates(&self) -> &GridCords3D {
        &self.grid_coordinate
    }

    pub fn get_name(&self) -> &String {
        &self.npc.get_name()
    }
}

// Returns a SpriteBundle for some tile
pub fn get_sprite_bundle(
    tile: &Tile,
    map_dimensions: &PxDimensions,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> SpriteBundle {
    // We have to trim our path from being absolute to having root at assets
    let bevy_path = to_bevy_path(&tile.tile_texture.as_ref().unwrap().spritesheet);
    let texture = asset_server.load(bevy_path);

    // Set the physical coordinates.
    let transform = Transform::from_xyz(
        tile.px_cords.px_x as f32,
        //y-axis flip because Bevy is Y-Up while Tiled is Y-Down
        flip_y_axis(
            map_dimensions.px_height,
            tile.px_cords.px_y as f32,
            tile.tile_dimensions.px_height,
        ),
        tile.px_cords.px_z as f32,
    );

    let mut sprite = Sprite::default();

    sprite.image = texture;

    let mut visibility = Visibility::Visible;
    let tile_type = tile.get_properties().get("type").unwrap();

    if *tile_type == String::from("Collision") {
        visibility = Visibility::Hidden;
    }

    let asset_id = UntypedAssetId::from(&sprite.image);
    let load_status = LoadStatus(asset_id);

    let texture_atlas = get_texture_atlas(tile, texture_atlas_assets);
    sprite.texture_atlas = Some(texture_atlas);

    let sprite_bundle = SpriteBundle::new(sprite, visibility, transform, load_status);
    sprite_bundle
}

/// Returns a TextureAtlas for some Tile.
fn get_texture_atlas(
    tile: &Tile,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> TextureAtlas {
    let mut texture_atlas = TextureAtlas::default();

    if tile.get_tile_texture().is_none() {
        return texture_atlas;
    }

    // Getting Spritesheet Dimensions
    let sprite_sheet_column_count =
        tile.get_spritesheet_dimensions().get_width() / tile.tile_dimensions.px_width;
    let sprite_sheet_row_count =
        tile.get_spritesheet_dimensions().get_height() / tile.tile_dimensions.px_height;

    // This is how the sprite sheet should be cut when creating sprites to render
    let sheet_layout = TextureAtlasLayout::from_grid(
        UVec2::new(
            tile.tile_dimensions.px_width as u32,
            tile.tile_dimensions.px_height as u32,
        ),
        sprite_sheet_column_count as u32,
        sprite_sheet_row_count as u32,
        None,
        None,
    );

    // And finally, in the spritesheet, we specify _which_ sprite in the spritesheet to render right now*.
    texture_atlas.layout = texture_atlas_assets.add(sheet_layout);

    // * specifically happening right here.
    texture_atlas.index = tile.tile_texture.as_ref().unwrap().sprite_index;

    texture_atlas
}

pub fn to_bevy_path(tiled_path: &Path) -> PathBuf {
    let mut trimmed_path = PathBuf::new();
    let mut path_element_stack = Vec::new();

    for path_element in tiled_path.iter().rev() {
        if path_element == "assets" {
            break;
        }

        path_element_stack.push(path_element);
    }

    while let Some(path_element) = path_element_stack.pop() {
        trimmed_path.push(path_element);
    }

    trimmed_path
}
