use std::fmt::Debug;

use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;

use cucumber::{given, then, when, World};

use tiled::{Loader, Map};

use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
    pub map_location: PathBuf,
    pub assets_folder_path: PathBuf,

    pub loaded_map: Tilemap,
    pub bevy_map: RenderedMap,
}

#[derive(Debug)]
struct Tilemap {
    tiled_tiles: Vec<Tile>,
    grid_dimensions: MapGridDimensions,
    px_dimensions: PxDimensions,
}

impl Tilemap {
    pub fn new(map_location: PathBuf) -> Self {
        let mut loader = Loader::new();
        let tiled_map = loader.load_tmx_map(map_location).unwrap();
        let num_layers = tiled_map.layers().len() as u32;

        let px_dimensions = Self::get_map_in_px(&tiled_map);
        let tiled_tiles = get_map_tiles(tiled_map);

        let num_rows = get_num_rows_from_map(&tiled_tiles);
        let num_columns = get_num_columns_from_map(&tiled_tiles);
        let grid_dimensions = MapGridDimensions::new(num_rows, num_columns, num_layers);

        Self {
            tiled_tiles,
            grid_dimensions,
            px_dimensions,
        }
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiled_tiles
    }

    pub fn get_grid_dimensions(&self) -> &MapGridDimensions {
        &self.grid_dimensions
    }

    pub fn get_px_dimensions(&self) -> &PxDimensions {
        &self.px_dimensions
    }

    pub fn tiles_overlap(&self, first_tile_index: usize, second_tile_index: usize) -> bool {
        let first_tile_px_position = &self.tiled_tiles[first_tile_index].px_cords;
        let second_tile_px_position = &self.tiled_tiles[second_tile_index].px_cords;

        if first_tile_px_position.px_x == second_tile_px_position.px_x
            && first_tile_px_position.px_y == second_tile_px_position.px_y
        {
            true
        } else {
            false
        }
    }

    fn get_map_in_px(tiled_map: &Map) -> PxDimensions {
        let px_dimensions = PxDimensions::new(
            tiled_map.width * &tiled_map.tile_width,
            tiled_map.height * &tiled_map.tile_height,
        );

        px_dimensions
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        Self {
            tiled_tiles: Vec::new(),
            grid_dimensions: MapGridDimensions::new(0, 0, 0),
            px_dimensions: PxDimensions::new(0, 0),
        }
    }
}

#[derive(Debug, PartialEq)]
struct XyzCords {
    px_x: usize,
    px_y: usize,
    px_z: usize,
}

impl XyzCords {
    pub fn new(px_x: u32, px_y: u32, px_z: usize) -> Self {
        XyzCords {
            px_x: px_x as usize,
            px_y: px_y as usize,
            px_z,
        }
    }
}

#[derive(Debug, PartialEq)]
struct PxDimensions {
    px_width: usize,
    px_height: usize,
}

impl PxDimensions {
    pub fn new(px_width: u32, px_height: u32) -> Self {
        PxDimensions {
            px_width: px_width as usize,
            px_height: px_height as usize,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MapGridDimensions {
    rows: u32,
    columns: u32,
    layers: u32,
}

impl MapGridDimensions {
    pub fn new(rows: u32, columns: u32, layers: u32) -> MapGridDimensions {
        MapGridDimensions {
            rows,
            columns,
            layers,
        }
    }

    pub fn get_layers(&self) -> u32 {
        self.layers
    }
}

#[derive(Debug)]
struct Tile {
    tile_dimensions: PxDimensions,
    px_cords: XyzCords,
    tile_texture: Option<TileTexture>,
    layer_number: usize,
}

impl Tile {
    pub fn new(
        tile_dimensions: PxDimensions,
        px_cords: XyzCords,
        tile_texture: Option<TileTexture>,
        layer_number: usize,
    ) -> Tile {
        Tile {
            tile_dimensions,
            px_cords,
            tile_texture,
            layer_number,
        }
    }

    pub fn get_tile_spritesheet_filename(&self) -> OsString {
        let tiles_texture = self.tile_texture.as_ref().unwrap();
        let spritesheet_name = tiles_texture.spritesheet.file_name().unwrap();

        spritesheet_name.into()
    }

    pub fn get_spritesheet_dimensions(&self) -> &PxDimensions {
        let tile_texture = self.tile_texture.as_ref().unwrap();
        &tile_texture.spritesheet_dimensions
    }

    pub fn get_sprite_index(&self) -> usize {
        self.tile_texture.as_ref().unwrap().sprite_index
    }
}

#[derive(Debug)]
struct TileTexture {
    spritesheet: PathBuf,
    sprite_index: usize,
    spritesheet_dimensions: PxDimensions,
}

#[derive(Bundle)]
struct RenderTile {
    spritesheet_bundle: SpriteSheetBundle,
}

#[derive(Default)]
struct RenderedMap {
    bevy_tiles: Vec<Option<RenderTile>>,
}

impl Debug for RenderedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderedMap")
            .field(
                "bevy_tiles",
                &format_args!(
                    "{}",
                    self.bevy_tiles.iter().filter(|tile| tile.is_some()).count()
                ),
            )
            .finish()
    }
}

impl RenderedMap {
    pub fn new(
        tilemap: &Tilemap,
        asset_server: &AssetServer,
        texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        RenderedMap {
            bevy_tiles: get_render_tile_bundles(&tilemap, &asset_server, texture_atlas_assets),
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
            .as_ref()
            .unwrap()
            .spritesheet_bundle
            .transform
            .translation;

        if tiled_tile_px_position.px_x == bevy_tile_px_position.x as usize
            && tiled_tile_px_position.px_y == bevy_tile_px_position.y as usize
        {
            true
        } else {
            false
        }
    }
}

impl GameWorld {
    pub fn new() -> Self {
        let map_location = PathBuf::new();
        let loaded_map = Tilemap::default();
        let bevy_map = RenderedMap::default();

        let absolute_assets_folder_path = PathBuf::new();

        // Testable "game"
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        });
        app.add_plugins(SpritePlugin::default());
        app.add_plugins(ImagePlugin::default());

        Self {
            app,
            map_location,
            loaded_map,
            bevy_map,
            assets_folder_path: absolute_assets_folder_path,
        }
    }
}

fn get_map_tiles(tiled_map: Map) -> Vec<Tile> {
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;

    let map_width = tiled_map.width;
    let map_height = tiled_map.height;

    let mut tiles = Vec::new();

    for z in 0..tiled_map.layers().len() {
        for y in 0..map_height {
            for x in 0..map_width {
                let tile = Tile {
                    tile_dimensions: PxDimensions::new(tile_width, tile_height),
                    px_cords: XyzCords::new(x * tile_width, y * tile_height, z),
                    layer_number: z,
                    tile_texture: get_tile_texture(&tiled_map, x, y, z),
                };
                tiles.push(tile);
            }
        }
    }

    tiles
}

/// Returns a list of RenderTileBundles to be spawned by Bevy for the given list of tiles.
fn get_render_tile_bundles(
    tilemap: &Tilemap,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> Vec<Option<RenderTile>> {
    let mut render_tile_bundles = Vec::new();

    let tiles = tilemap.get_tiles();

    for tile in tiles {
        //Tiles without a texture don't need to be rendered
        if tile.tile_texture.is_none() {
            render_tile_bundles.push(None);
            continue;
        }

        //We have to trim our path from being absolute to having root at assets
        let bevy_path = to_bevy_path(&tile.tile_texture.as_ref().unwrap().spritesheet);
        let texture = asset_server.load(bevy_path);

        //Getting Spritesheet Dimensions
        let sprite_sheet_column_count = (tile
            .tile_texture
            .as_ref()
            .unwrap()
            .spritesheet_dimensions
            .px_width
            / tile.tile_dimensions.px_width) as usize;
        let sprite_sheet_row_count = (tile
            .tile_texture
            .as_ref()
            .unwrap()
            .spritesheet_dimensions
            .px_height
            / tile.tile_dimensions.px_height) as usize;

        //This is how the sprite sheet should be cut when creating sprites to render
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

        // Conversion to Bevy specific formatting happens right here
        // Our:RenderTileBundle -> Bevy's:SpriteSheetBundle
        let render_tile = Some(RenderTile {
            spritesheet_bundle: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    tile.px_cords.px_x as f32,
                    //y-axis flip because Bevy is Y-Up while Tiled is Y-Down
                    flip_y_axis(
                        tilemap.get_px_dimensions().px_height,
                        tile.px_cords.px_y as f32,
                        tile.tile_dimensions.px_height,
                    ),
                    tile.px_cords.px_z as f32,
                ),
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_assets.add(sheet_layout),
                    index: tile.tile_texture.as_ref().unwrap().sprite_index,
                },
                ..Default::default()
            },
        });
        render_tile_bundles.push(render_tile);
    }
    render_tile_bundles
}

fn get_tile_texture(
    tiled_map: &Map,
    x_grid_cord: u32,
    y_grid_cord: u32,
    layer_num: usize,
) -> Option<TileTexture> {
    let tile_layer = tiled_map
        .get_layer(layer_num)
        .unwrap()
        .as_tile_layer()
        .unwrap();

    if let Some(tile) = tile_layer.get_tile(x_grid_cord as i32, y_grid_cord as i32) {
        tile_layer
            .get_tile(x_grid_cord as i32, y_grid_cord as i32)
            .unwrap();

        let sprite_index = tile.id() as usize;
        let spritesheet = tile.get_tileset().image.clone().unwrap().source;
        let spritesheet_px_width = tile.get_tileset().image.as_ref().unwrap().width as u32;
        let spritesheet_px_height = tile.get_tileset().image.as_ref().unwrap().height as u32;

        Some(TileTexture {
            sprite_index,
            spritesheet,
            spritesheet_dimensions: PxDimensions::new(spritesheet_px_width, spritesheet_px_height),
        })
    } else {
        None
    }
}

fn to_bevy_path(tiled_path: &PathBuf) -> PathBuf {
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

    return trimmed_path;
}

fn flip_y_axis(map_height: usize, tile_y: f32, tile_height: usize) -> f32 {
    let flipped_y = map_height as f32 - tile_y - tile_height as f32;

    return flipped_y;
}

// Returns a Path to the specified Tiled Map
// for a testing environment.
fn get_tiled_map_location(map_name: String) -> PathBuf {
    let mut tiled_map_path = PathBuf::new();

    // Appends the Manifest Directory which represents the root of the whole project.
    // We need this since we cannot use relative paths for testing purposes.
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        tiled_map_path.push(manifest_dir);
    }

    tiled_map_path.push("tests/test-assets/maps");
    tiled_map_path.push(map_name);

    tiled_map_path
}

fn get_num_columns_from_map(tiles: &[Tile]) -> u32 {
    let mut highest_x = 0;
    let tile_width = tiles[0].tile_dimensions.px_width;

    for tile in tiles {
        if tile.px_cords.px_x >= highest_x {
            highest_x = tile.px_cords.px_x;
        } else {
            break;
        }
    }

    ((highest_x / tile_width) + 1) as u32
}

fn get_num_rows_from_map(tiles: &[Tile]) -> u32 {
    let mut highest_y = 0;
    let tile_height = tiles[0].tile_dimensions.px_height;

    for tile in tiles {
        if tile.px_cords.px_y >= highest_y {
            highest_y = tile.px_cords.px_y;
        } else {
            break;
        }
    }

    ((highest_y / tile_height) + 1) as u32
}

//////////////TEST FUNCTIONS//////////////

#[given(regex = r"a Tiled map called (.+\.tmx),")]
fn verify_test_map_exists(world: &mut GameWorld, map_name: String) {
    let unloaded_tiled_map = get_tiled_map_location(map_name);

    assert!(
        unloaded_tiled_map.exists(),
        "File does not exist at location {:?}",
        unloaded_tiled_map.canonicalize().unwrap()
    );

    world.map_location = unloaded_tiled_map;
}

#[given(regex = r"an absolute asset path of (.+\.png),")]
fn set_absolute_path(world: &mut GameWorld, absolute_asset_path: String) {
    world.assets_folder_path = PathBuf::from(absolute_asset_path);
}

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    world.loaded_map = Tilemap::new(world.map_location.clone());
}

#[when("the absolute path is trimmed,")]
fn trim_to_bevy_path(world: &mut GameWorld) {
    let original_path = &world.assets_folder_path;
    let trimmed_path = to_bevy_path(original_path);

    world.assets_folder_path = trimmed_path;
}

#[when("the Tiled map has been converted to a rendered map,")]
fn tiled_map_to_bevy_tiles(world: &mut GameWorld) {
    let tilemap = &world.loaded_map;
    let asset_server = world.app.world.resource::<AssetServer>().clone();
    let mut texture_atlas_layout = world.app.world.resource_mut::<Assets<TextureAtlasLayout>>();

    let rendered_bevy_map = RenderedMap::new(&tilemap, &asset_server, &mut texture_atlas_layout);
    world.bevy_map = rendered_bevy_map;
}

#[then(regex = r"there are ([0-9]+) tiles loaded.")]
fn verify_num_loaded_tiles(world: &mut GameWorld, map_tile_count: String) {
    let expected_num_tiles = map_tile_count
        .parse::<usize>()
        .expect("verify_num_loaded_tiles: map_tile_count is not a number?");
    let actual_num_tiles = world.loaded_map.get_tiles().len();
    assert_eq!(expected_num_tiles, actual_num_tiles);
}

#[then(regex = r"the tiles are in a ([0-9]+)x([0-9]+) grid.")]
fn verify_tiles_are_a_grid(world: &mut GameWorld, column_num: String, row_num: String) {
    let expected_num_rows = column_num
        .parse::<u32>()
        .expect("verify_tiles_are_a_grid: column is not a number?");

    let expected_num_columns = row_num
        .parse::<u32>()
        .expect("verify_tiles_are_a_grid: row is not a number?");

    let actual_map_dimensions = world.loaded_map.get_grid_dimensions();
    let expected_map_dimensions =
        MapGridDimensions::new(expected_num_rows, expected_num_columns, 1);

    assert_eq!(expected_map_dimensions, *actual_map_dimensions);
}

#[then(regex = r"tile ([0-9]+) points to spritesheet (.+\.png).")]
fn verify_tile_spritesheet(world: &mut GameWorld, tile_num: String, spritesheet_name: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_tile_spritesheet: tile_num is not a number?");

    let actual_spritesheet =
        world.loaded_map.get_tiles()[tile_index - 1].get_tile_spritesheet_filename();
    let expected_spritesheet = OsString::from(spritesheet_name);
    assert_eq!(expected_spritesheet, actual_spritesheet);
}

#[then(regex = r"tile ([0-9]+)'s spritesheet has dimensions ([0-9]+) x ([0-9]+).")]
fn verify_spritesheet_dimensions(
    world: &mut GameWorld,
    tile_num: String,
    sheet_height: String,
    sheet_width: String,
) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_spritesheet_dimensions: tile_num is not a number?");

    let expected_spritesheet_height = sheet_height
        .parse::<u32>()
        .expect("verify_spritesheet_dimensions: sheet_height is not a number?");

    let expected_spritesheet_width = sheet_width
        .parse::<u32>()
        .expect("verify_spritesheet_dimensions: sheet_width is not a number?");

    let actual_dimensions =
        world.loaded_map.get_tiles()[tile_index - 1].get_spritesheet_dimensions();
    let expected_dimensions =
        PxDimensions::new(expected_spritesheet_width, expected_spritesheet_height);
    assert_eq!(expected_dimensions, *actual_dimensions);
}

#[then(regex = r"tile ([0-9]+) points to image number ([0-9]+).")]
fn verify_spritesheet_tile_image(world: &mut GameWorld, tile_num: String, image_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_spritesheet_tile_image: tile_num is not a number?");
    let image_index = image_num
        .parse::<usize>()
        .expect("verify_spritesheet_tile_image: image_num is not a number?");

    let actual_image = world.loaded_map.get_tiles()[tile_index - 1].get_sprite_index();
    let expected_image = image_index;
    assert_eq!(expected_image, actual_image);
}

#[then(regex = r"tile ([0-9]+) contains an image element.")]
fn verify_tile_contains_image(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_tile_contains_image: tile_num is not a number?");

    let tile_image = &world.loaded_map.tiled_tiles[tile_index - 1].tile_texture;

    assert!(tile_image.is_some());
}

#[then(regex = r"tile ([0-9]+) contains no image element.")]
fn verify_tile_image_is_empty(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_tile_contains_image: tile_num is not a number?");

    let tile_image = &world.loaded_map.get_tiles()[tile_index - 1].tile_texture;
    assert!(tile_image.is_none());
}

#[then(regex = r"there exist ([0-9]+) layers of tiles.")]
fn verify_layer_count(world: &mut GameWorld, num_layers: String) {
    let layer_count = num_layers
        .parse::<u32>()
        .expect("verify_layer_count: num_layers is not a number?");

    let actual_num_layers = world.loaded_map.get_grid_dimensions().get_layers();
    let expected_num_layers = layer_count;
    assert_eq!(expected_num_layers, actual_num_layers);
}

#[then(regex = r"tile ([0-9]+) overlaps tile ([0-9]+).")]
fn verify_overlapping_tiles(world: &mut GameWorld, first_tile: String, second_tile: String) {
    let first_tile_index = first_tile
        .parse::<usize>()
        .expect("verify_overlapping_tiles: first_tile is not a number?");
    let second_tile_index = second_tile
        .parse::<usize>()
        .expect("verify_overlapping_tiles: second_tile is not a number?");

    let is_overlapping = world
        .loaded_map
        .tiles_overlap(first_tile_index - 1, second_tile_index - 1);

    assert!(is_overlapping);
}

#[then(regex = r"Tiled tile ([0-9]+) overlaps Bevy tile ([0-9]+)")]
fn verify_y_axis_flip(world: &mut GameWorld, tiled_tile_num: String, bevy_tile_num: String) {
    let tiled_tile_index = tiled_tile_num
        .parse::<usize>()
        .expect("verify_y_axis_flip: Tiled tile is not a number?");

    let bevy_tile_index = bevy_tile_num
        .parse::<usize>()
        .expect("verify_y_axis_flip: Bevy tile is not a number?");

    let tiled_map = &world.loaded_map;

    let cross_map_overlap =
        world
            .bevy_map
            .tiled_map_overlap(tiled_map, tiled_tile_index - 1, bevy_tile_index - 1);

    assert!(cross_map_overlap);
}

#[then(regex = r"tile ([0-9]+) is in the rendered map.")]
fn verify_render_tile_is_some(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_render_tile_is_some: tile_num is not a number?");

    let rendered_tile = &world.bevy_map.bevy_tiles[tile_index - 1];
    assert!(rendered_tile.is_some());
}

#[then(regex = r"tile ([0-9]+) is not in the rendered map.")]
fn verify_render_tile_is_none(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_render_tile_is_non: tile_num is not a number?");

    let rendered_tile = &world.bevy_map.bevy_tiles[tile_index - 1];
    assert!(rendered_tile.is_none())
}

#[then(regex = r"the trimmed path should be (.+\.png).")]
fn verify_trimmed_path(world: &mut GameWorld, desired_asset_path: String) {
    let expected_path = PathBuf::from(desired_asset_path);
    let actual_path = &world.assets_folder_path;

    assert_eq!(expected_path, *actual_path);
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}
