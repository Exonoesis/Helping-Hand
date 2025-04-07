use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use bevy::prelude::*;

use super::{flip_y_axis, GridDimensions, PxDimensions, Tile, TileType, Tilemap, XyzCords};

#[derive(Bundle, Clone)]
pub struct RenderTile {
    grid_coordinate: XyzCords,
    tile_type: TileType,
    tile_dimensions: PxDimensions,
    spritesheet_bundle: SpriteSheetBundle,
}

impl RenderTile {
    pub fn new(
        grid_coordinate: XyzCords,
        tile_type: TileType,
        tile_dimensions: PxDimensions,
        spritesheet: SpriteSheetBundle,
    ) -> Self {
        Self {
            grid_coordinate,
            tile_type,
            tile_dimensions,
            spritesheet_bundle: spritesheet,
        }
    }

    pub fn get_tile_type(&self) -> &TileType {
        &self.tile_type
    }

    pub fn is_invisible(&self) -> bool {
        let is_invisible = self.spritesheet_bundle.visibility == Visibility::Hidden;

        is_invisible
    }

    pub fn get_grid_coordinates(&self) -> &XyzCords {
        &self.grid_coordinate
    }

    pub fn get_tile_dimensions(&self) -> &PxDimensions {
        &self.tile_dimensions
    }
}

#[derive(Default)]
pub struct RenderedMap {
    map_dimensions_in_px: PxDimensions,
    grid_dimensions: GridDimensions,
    bevy_tiles: Vec<RenderTile>,
}

impl Debug for RenderedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderedMap")
            .field("bevy_tiles", &format_args!("{}", self.bevy_tiles.len()))
            .finish()
    }
}

impl RenderedMap {
    pub fn new(
        tilemap: &Tilemap,
        asset_server: &AssetServer,
        texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let map_dimensions_in_px = *tilemap.get_px_dimensions();
        let grid_dimensions = *tilemap.get_grid_dimensions();
        RenderedMap {
            map_dimensions_in_px,
            grid_dimensions,
            bevy_tiles: get_render_tile_bundles(tilemap, asset_server, texture_atlas_assets),
        }
    }

    pub fn tiled_map_overlap(
        &self,
        tiled_map: &Tilemap,
        tiled_tile_index: usize,
        bevy_tile_index: usize,
    ) -> bool {
        let tiled_tile_px_position = &tiled_map.get_tiles()[tiled_tile_index].px_cords;
        let bevy_tile_px_position = &self.bevy_tiles[bevy_tile_index]
            .spritesheet_bundle
            .transform
            .translation;

        tiled_tile_px_position.px_x == bevy_tile_px_position.x as usize
            && tiled_tile_px_position.px_y == bevy_tile_px_position.y as usize
    }

    pub fn get_bevy_tiles(&self) -> &Vec<RenderTile> {
        &self.bevy_tiles
    }

    pub fn get_px_dimensions(&self) -> &PxDimensions {
        &self.map_dimensions_in_px
    }

    pub fn get_grid_dimensions(&self) -> &GridDimensions {
        &self.grid_dimensions
    }
}

/// Returns a list of RenderTileBundles to be spawned by Bevy for the given list of tiles.
fn get_render_tile_bundles(
    tilemap: &Tilemap,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> Vec<RenderTile> {
    let mut render_tile_bundles = Vec::new();

    let tiles = tilemap.get_tiles();

    for tile in tiles {
        // Conversion to Bevy specific formatting happens right here
        // Our:RenderTileBundle -> Bevy's:SpriteSheetBundle
        let render_tile_spritesheet =
            get_spritesheet_for_tile(tile, tilemap, asset_server, texture_atlas_assets);
        let render_tile_coordinate = tile.get_grid_coordinates();
        let render_tile_dimensions = tile.get_tile_dimensions();
        let render_tile_type = tile.get_tile_type();
        let render_tile = RenderTile::new(
            *render_tile_coordinate,
            *render_tile_type,
            *render_tile_dimensions,
            render_tile_spritesheet,
        );
        render_tile_bundles.push(render_tile);
    }
    render_tile_bundles
}

/// Returns a Spritesheet for some Tile.
fn get_spritesheet_for_tile(
    tile: &Tile,
    tilemap: &Tilemap,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> SpriteSheetBundle {
    let mut tile_spritesheet = SpriteSheetBundle::default();

    if tile.get_tile_texture().is_none() {
        tile_spritesheet.transform = Transform::from_xyz(
            tile.px_cords.px_x as f32,
            // Y-axis flip, because Bevy is Y-Up while Tiled is Y-Down
            flip_y_axis(
                tilemap.get_px_dimensions().px_height,
                tile.px_cords.px_y as f32,
                tile.tile_dimensions.px_height,
            ),
            tile.px_cords.px_z as f32,
        );
        tile_spritesheet.visibility = Visibility::Hidden;

        return tile_spritesheet;
    }

    // We have to trim our path from being absolute to having root at assets
    let bevy_path = to_bevy_path(&tile.tile_texture.as_ref().unwrap().spritesheet);
    let texture = asset_server.load(bevy_path);

    // Getting Spritesheet Dimensions
    let sprite_sheet_column_count =
        tile.get_spritesheet_dimensions().get_width() / tile.tile_dimensions.px_width;
    let sprite_sheet_row_count =
        tile.get_spritesheet_dimensions().get_height() / tile.tile_dimensions.px_height;

    // This is how the sprite sheet should be cut when creating sprites to render
    let sheet_layout = TextureAtlasLayout::from_grid(
        Vec2::new(
            tile.tile_dimensions.px_width as f32,
            tile.tile_dimensions.px_height as f32,
        ),
        sprite_sheet_column_count,
        sprite_sheet_row_count,
        None,
        None,
    );

    // First we set the physical coordinates.
    tile_spritesheet.transform = Transform::from_xyz(
        tile.px_cords.px_x as f32,
        //y-axis flip because Bevy is Y-Up while Tiled is Y-Down
        flip_y_axis(
            tilemap.get_px_dimensions().px_height,
            tile.px_cords.px_y as f32,
            tile.tile_dimensions.px_height,
        ),
        tile.px_cords.px_z as f32,
    );
    // Then we point to the spritesheet file to use as reference.
    tile_spritesheet.texture = texture;
    // And finally, in the spritesheet, we specify _which_ sprite in the spritesheet to render right now*.
    tile_spritesheet.atlas = TextureAtlas {
        layout: texture_atlas_assets.add(sheet_layout),
        // * specifically happening right here.
        index: tile.tile_texture.as_ref().unwrap().sprite_index,
    };

    let tile_type = tile.get_tile_type();

    if tile_type == &TileType::Collision {
        tile_spritesheet.visibility = Visibility::Hidden;
    }

    tile_spritesheet
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
