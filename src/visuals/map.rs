use std::collections::{HashMap, HashSet};
use std::{fmt::Debug, path::Path};

use std::ffi::OsString;
use std::path::PathBuf;

use tiled::{Loader, Map, ObjectShape, PropertyValue};

use bevy::prelude::*;

use crate::entities::player::Player;
use crate::mechanics::input::MovementDirection;

#[derive(Event)]
pub struct ChangeLevel {
    level_name: String,
}

impl ChangeLevel {
    pub fn new(desired_level_name: &str) -> Self {
        Self {
            level_name: String::from(desired_level_name),
        }
    }

    pub fn get_level_path(&self) -> &str {
        &self.level_name
    }

    pub fn get_level_name(&self) -> String {
        let level_path = PathBuf::from(self.get_level_path());

        let level_name = level_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        level_name
    }
}

/// Returns a camera centered to some map.
fn create_centered_camera(map: &Tilemap) -> Camera2dBundle {
    let mut the_camera = Camera2dBundle::default();
    let camera_position = &mut the_camera.transform;

    let horizontal_center = (map.get_px_dimensions().get_width() / 2) as f32;
    let vertical_center = (map.get_px_dimensions().get_height() / 2) as f32;

    *camera_position = Transform::from_xyz(horizontal_center, vertical_center, 999.0);

    the_camera
}

/// Loads some predetermined map when clicking the "Play" button.
pub fn load_starting_map(mut change_level_requester: EventWriter<ChangeLevel>) {
    let tiled_map_name = "test_map_with_collision.tmx";
    let map_path = format!("tests/test-assets/maps/{}", tiled_map_name);
    change_level_requester.send(ChangeLevel::new(&map_path));
}

/// Unloads the current Tiled map.
fn despawn_level(
    loaded_level_tiles: Query<(Entity, &XyzCords, &TileType, &PxDimensions)>,
    map_properties: Query<
        Entity,
        (
            With<PxDimensions>,
            With<InteractiveCollection>,
            With<CollisionCollection>,
            With<GridDimensions>,
        ),
    >,
    camera: Query<Entity, With<Camera2d>>,
    commands: &mut Commands,
) {
    for loaded_tile in &loaded_level_tiles {
        let loaded_tile_entity = loaded_tile.0;
        commands.entity(loaded_tile_entity).despawn_recursive();
    }

    let camera_entity = camera.single();
    commands.entity(camera_entity).despawn_recursive();

    let map_properties_entity = map_properties.single();
    commands.entity(map_properties_entity).despawn_recursive();
}

/// Returns a loaded Tiled map.
pub fn load_tiled_map(map_location: PathBuf) -> Map {
    let mut loader = Loader::new();
    loader.load_tmx_map(map_location).unwrap()
}

/// Loads the Tiled test map with a Camera into the game at the center of the map.
pub fn load_map(
    mut change_level_requests: EventReader<ChangeLevel>,
    mut commands: Commands,
    asset_spawner: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlasLayout>>,
    loaded_level_tiles: Query<(Entity, &XyzCords, &TileType, &PxDimensions)>,
    camera: Query<Entity, With<Camera2d>>,
    map_properties: Query<
        Entity,
        (
            With<PxDimensions>,
            With<InteractiveCollection>,
            With<CollisionCollection>,
            With<GridDimensions>,
        ),
    >,
) {
    if change_level_requests.is_empty() {
        return;
    }

    let map_already_loaded = loaded_level_tiles.iter().len() > 0;
    if map_already_loaded {
        despawn_level(loaded_level_tiles, map_properties, camera, &mut commands);
    }

    let change_level_request = change_level_requests
        .read()
        .next()
        .expect("load_map: No change level events found.");
    let tiled_map = load_tiled_map(PathBuf::from(change_level_request.get_level_path()));
    let map = Tilemap::from_tiled(&tiled_map);
    let bevy_map = RenderedMap::new(&map, &asset_spawner, &mut texture_atlas_assets);

    let rendered_tiles = bevy_map.get_bevy_tiles();

    for render_tile in rendered_tiles {
        let render_tile = render_tile.clone();
        if render_tile.get_tile_type() == &TileType::Player {
            commands.spawn((render_tile, Player, MovementDirection::Left));
            continue;
        }

        commands.spawn(render_tile);
    }

    let camera_centered_to_map = create_centered_camera(&map);
    commands.spawn(camera_centered_to_map);

    // This section represents the Physical properties of the map.
    let map_size_in_px = *bevy_map.get_px_dimensions();
    let map_grid_dimenions = *bevy_map.get_grid_dimensions();
    let mut interactives = get_interactives_from(&tiled_map);
    // We have to flip the y-axis of all tiles, since they're physical coordinates.
    interactives = flip_interactives_on_y_axis(interactives, map_size_in_px, map_grid_dimenions);
    let interactive_collection = InteractiveCollection::from_markers(interactives);
    let physical_properties = (map_size_in_px, interactive_collection);

    // This section represents all of the Logical properties of the map.
    let collision_collection = create_collision_collection_from(&bevy_map);
    let map_size_in_tiles = *bevy_map.get_grid_dimensions();
    let logical_properties = (collision_collection, map_size_in_tiles);

    commands.spawn((physical_properties, logical_properties));
}

#[derive(Debug)]
pub struct Tilemap {
    tiled_tiles: Vec<Tile>,
    grid_dimensions: GridDimensions,
    px_dimensions: PxDimensions,
}

impl Tilemap {
    pub fn from_tiled(tiled_map: &Map) -> Self {
        let num_layers = tiled_map.layers().len() as u32;

        let px_dimensions = Self::get_map_in_px(&tiled_map);
        let tiled_tiles = get_map_tiles(&tiled_map);

        let num_rows = get_num_rows_from_map(&tiled_tiles);
        let num_columns = get_num_columns_from_map(&tiled_tiles);
        let grid_dimensions = GridDimensions::new(num_columns, num_rows, num_layers);

        Self {
            tiled_tiles,
            grid_dimensions,
            px_dimensions,
        }
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiled_tiles
    }

    pub fn get_grid_dimensions(&self) -> &GridDimensions {
        &self.grid_dimensions
    }

    pub fn get_px_dimensions(&self) -> &PxDimensions {
        &self.px_dimensions
    }

    pub fn tiles_overlap(&self, first_tile_index: usize, second_tile_index: usize) -> bool {
        let first_tile_px_position = &self.tiled_tiles[first_tile_index].px_cords;
        let second_tile_px_position = &self.tiled_tiles[second_tile_index].px_cords;

        first_tile_px_position.px_x == second_tile_px_position.px_x
            && first_tile_px_position.px_y == second_tile_px_position.px_y
    }

    fn get_map_in_px(tiled_map: &Map) -> PxDimensions {
        PxDimensions::new(
            tiled_map.width * tiled_map.tile_width,
            tiled_map.height * tiled_map.tile_height,
        )
    }

    pub fn get_players(&self) -> Vec<&Tile> {
        let mut found_players = Vec::new();

        for tile in &self.tiled_tiles {
            if *tile.get_tile_type() == TileType::Player {
                found_players.push(tile);
            }
        }

        found_players
    }
}

impl Default for Tilemap {
    fn default() -> Self {
        Self {
            tiled_tiles: Vec::new(),
            grid_dimensions: GridDimensions::new(0, 0, 0),
            px_dimensions: PxDimensions::new(0, 0),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Normal,
    Player,
    Collision,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XyzCords {
    px_x: usize,
    px_y: usize,
    px_z: usize,
}

impl XyzCords {
    pub fn new(px_x: usize, px_y: usize, px_z: usize) -> Self {
        XyzCords { px_x, px_y, px_z }
    }

    pub fn new_u32(px_x: u32, px_y: u32, px_z: usize) -> Self {
        Self::new(px_x as usize, px_y as usize, px_z)
    }

    pub fn get_x(&self) -> usize {
        self.px_x
    }

    pub fn get_y(&self) -> usize {
        self.px_y
    }

    pub fn get_z(&self) -> usize {
        self.px_z
    }
}

// This function loses floating point accuracy
pub fn transform_to_xyzcord(transform: Transform) -> XyzCords {
    XyzCords::new(
        transform.translation.x as usize,
        transform.translation.y as usize,
        transform.translation.z as usize,
    )
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PxDimensions {
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

    pub fn get_width(&self) -> usize {
        self.px_width
    }

    pub fn get_height(&self) -> usize {
        self.px_height
    }
}

#[derive(Component, Copy, Clone, Debug, Default, PartialEq)]
pub struct GridDimensions {
    columns: u32,
    rows: u32,
    layers: u32,
}

impl GridDimensions {
    pub fn new(columns: u32, rows: u32, layers: u32) -> GridDimensions {
        GridDimensions {
            columns,
            rows,
            layers,
        }
    }

    pub fn get_layers(&self) -> u32 {
        self.layers
    }

    pub fn get_columns(&self) -> u32 {
        self.columns
    }

    pub fn get_rows(&self) -> u32 {
        self.rows
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Proximity {
    Lower,
    Higher,
    Match,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum InteractiveType {
    Transition(PathBuf),
}

impl InteractiveType {
    fn type_name(&self) -> String {
        match self {
            InteractiveType::Transition(_) => "Transition".to_string(),
        }
    }

    fn type_value(&self) -> PathBuf {
        match self {
            InteractiveType::Transition(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct InteractiveMarker {
    position: XyzCords,
    dimensions: PxDimensions,
    interaction_type: InteractiveType,
}

impl InteractiveMarker {
    pub fn new(
        position: XyzCords,
        dimensions: PxDimensions,
        interaction_type: InteractiveType,
    ) -> Self {
        InteractiveMarker {
            position,
            dimensions,
            interaction_type,
        }
    }

    pub fn containing(&self, position: &XyzCords) -> Proximity {
        let marker_min_x = self.position.get_x();
        let marker_max_x = marker_min_x + (self.dimensions.get_width());
        let marker_x_range = marker_min_x..marker_max_x;
        let position_x = position.get_x();

        let marker_min_y = self.position.get_y();
        let marker_max_y = marker_min_y + (self.dimensions.get_height());
        let marker_y_range = marker_min_y..marker_max_y;
        let position_y = position.get_y();

        if marker_x_range.contains(&position_x) && marker_y_range.contains(&position_y) {
            return Proximity::Match;
        }

        if position_x < marker_min_x {
            Proximity::Lower
        } else if position_x >= marker_max_x {
            Proximity::Higher
        } else if position_y < marker_min_y {
            Proximity::Lower
        } else {
            Proximity::Higher
        }
    }

    pub fn get_position(&self) -> XyzCords {
        self.position
    }

    pub fn get_dimensions(&self) -> PxDimensions {
        self.dimensions
    }

    pub fn get_interactive_type(&self) -> InteractiveType {
        self.interaction_type.clone()
    }

    pub fn get_type_name(&self) -> String {
        self.interaction_type.type_name()
    }

    pub fn get_path(&self) -> PathBuf {
        self.interaction_type.type_value()
    }
}

#[derive(Debug)]
pub struct Tile {
    tile_dimensions: PxDimensions,
    px_cords: XyzCords,
    grid_cords: XyzCords,
    tile_texture: Option<TileTexture>,
    layer_number: usize,
    tile_type: TileType,
}

impl Tile {
    pub fn new(
        tile_dimensions: PxDimensions,
        px_cords: XyzCords,
        grid_cords: XyzCords,
        tile_texture: Option<TileTexture>,
        layer_number: usize,
        tile_type: TileType,
    ) -> Tile {
        Tile {
            tile_dimensions,
            px_cords,
            grid_cords,
            tile_texture,
            layer_number,
            tile_type,
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

    pub fn get_tile_dimensions(&self) -> &PxDimensions {
        &self.tile_dimensions
    }

    pub fn get_grid_coordinates(&self) -> &XyzCords {
        &self.grid_cords
    }

    pub fn get_sprite_index(&self) -> usize {
        self.tile_texture.as_ref().unwrap().sprite_index
    }

    pub fn get_tile_texture(&self) -> &Option<TileTexture> {
        &self.tile_texture
    }

    pub fn get_tile_type(&self) -> &TileType {
        &self.tile_type
    }
}

#[derive(Debug)]
pub struct TileTexture {
    spritesheet: PathBuf,
    sprite_index: usize,
    spritesheet_dimensions: PxDimensions,
}

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

fn is_tile_layer(tiled_map: &Map, idx: usize) -> bool {
    let found_tile_layer = tiled_map.get_layer(idx).unwrap().as_tile_layer();

    found_tile_layer.is_some()
}

fn get_map_tiles(tiled_map: &Map) -> Vec<Tile> {
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;

    let map_width = tiled_map.width;
    let map_height = tiled_map.height;

    let mut tiles = Vec::new();

    for z in 0..tiled_map.layers().len() {
        let is_tile_layer = is_tile_layer(&tiled_map, z);
        if !is_tile_layer {
            continue;
        }

        for y in 0..map_height {
            for x in 0..map_width {
                let tile_dimensions = PxDimensions::new(tile_width, tile_height);
                let px_cords = XyzCords::new_u32(x * tile_width, y * tile_height, z);
                let grid_cords = XyzCords::new_u32(x, y, z);
                let layer_number = z;
                let tile_texture = get_tile_texture(&tiled_map, x, y, z);
                let tile_type = get_tile_type(&tiled_map, x, y, z);

                let tile = Tile::new(
                    tile_dimensions,
                    px_cords,
                    grid_cords,
                    tile_texture,
                    layer_number,
                    tile_type,
                );
                tiles.push(tile);
            }
        }
    }

    tiles
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

fn get_tile_type(
    tiled_map: &Map,
    x_grid_cord: u32,
    y_grid_cord: u32,
    layer_num: usize,
) -> TileType {
    let tile_layer = tiled_map.get_layer(layer_num).unwrap();
    let layer_name = tile_layer.name.clone();

    /*
     "This is equivalent to a pit-stop, where we grab name along the way." - Exo

    let tile_layer = tiled_map
        .get_layer(layer_num)
        .unwrap() <- tile_layer, before the variable shadowing occurs.
        .as_tile_layer() <- tile_layer, after the variable shadowing occurs, getting this part and onward without having to redine the whole thing.
        .unwrap();
     */
    let tile_layer = tile_layer.as_tile_layer().unwrap();

    let has_tile_at_layer = tile_layer
        .get_tile(x_grid_cord as i32, y_grid_cord as i32)
        .is_some();
    if !has_tile_at_layer {
        return TileType::Empty;
    }

    match layer_name.as_str() {
        "Player" => TileType::Player,
        "Collision" => TileType::Collision,
        _ => TileType::Normal,
    }
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

fn flip_y_axis(map_height: usize, tile_y: f32, tile_height: usize) -> f32 {
    map_height as f32 - tile_y - tile_height as f32
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

pub fn three_d_to_one_d_cords(
    tile_grid_cords: &GridDimensions,
    map_grid_dimensions: &GridDimensions,
) -> u32 {
    let map_area = map_grid_dimensions.columns * map_grid_dimensions.rows;
    let map_length = map_grid_dimensions.columns;
    let tile_x = tile_grid_cords.columns;
    let tile_y = tile_grid_cords.rows;
    let tile_z = tile_grid_cords.layers;

    (map_area * tile_z) + (map_length * tile_y) + tile_x
}

#[derive(Component, Debug, Default)]
pub struct CollisionCollection {
    collision_tiles: HashSet<XyzCords>,
}

impl CollisionCollection {
    pub fn new() -> Self {
        let collision_tiles = HashSet::new();

        Self { collision_tiles }
    }

    pub fn has(&self, xyz_coord: &XyzCords) -> bool {
        // NOTE: Collision should apply to all layers, thus the z value does
        // not make sense, hence it being zeroed out.
        let xy_coord = XyzCords::new(xyz_coord.get_x(), xyz_coord.get_y(), 0);
        self.collision_tiles.contains(&xy_coord)
    }

    pub fn add(&mut self, xyz_coord: &XyzCords) {
        let xy_coord = XyzCords::new(xyz_coord.get_x(), xyz_coord.get_y(), 0);
        self.collision_tiles.insert(xy_coord);
    }
}

pub fn create_collision_collection_from(bevy_map: &RenderedMap) -> CollisionCollection {
    let mut collision_collection = CollisionCollection::new();

    let rendered_tiles = bevy_map.get_bevy_tiles();
    for rendered_tile in rendered_tiles {
        if rendered_tile.get_tile_type() != &TileType::Collision {
            continue;
        }

        let rendered_tile_coord = rendered_tile.get_grid_coordinates();
        collision_collection.add(rendered_tile_coord);
    }

    collision_collection
}

#[derive(Component, Debug, Clone, Default)]
pub struct InteractiveCollection {
    interactive_markers: Vec<InteractiveMarker>,
}

impl InteractiveCollection {
    pub fn new() -> Self {
        let interactive_markers = Vec::new();

        Self {
            interactive_markers,
        }
    }

    pub fn from_markers(mut interactive_markers: Vec<InteractiveMarker>) -> Self {
        interactive_markers.sort();

        Self {
            interactive_markers,
        }
    }

    pub fn len(&self) -> usize {
        self.interactive_markers.len()
    }

    pub fn get_marker_at_index(&self, index: usize) -> &InteractiveMarker {
        &self.interactive_markers[index]
    }

    pub fn get_marker_from_position(&self, position: &XyzCords) -> Option<&InteractiveMarker> {
        if self.len() == 0 {
            return None;
        }

        let mut left = 0;
        let mut right = self.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let marker = self.get_marker_at_index(mid);
            let comparison = marker.containing(position);

            if comparison == Proximity::Higher {
                left = mid + 1;
            } else if comparison == Proximity::Lower {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                return Some(marker);
            }
        }
        None
    }
}

pub fn get_interactives_from(tiled_map: &Map) -> Vec<InteractiveMarker> {
    let mut interactive_markers = Vec::new();

    for z in 0..tiled_map.layers().len() {
        let found_object_layer = tiled_map.get_layer(z).unwrap().as_object_layer();

        if found_object_layer.is_none() {
            continue;
        }

        let object_layer = found_object_layer.unwrap();
        let objects = object_layer.objects();

        for object in objects {
            let position = XyzCords::new(object.x as usize, object.y as usize, z);

            // Get properties and create interactive type from it
            let properties = &object.properties;
            let interactive_type = create_interactive_type(properties);

            // Get shape, check it's a Rect, get width and height
            if let ObjectShape::Rect { width, height } = object.shape {
                let object_width = width as u32;
                let object_height = height as u32;

                let dimensions = PxDimensions::new(object_width, object_height);

                let interactive_marker =
                    InteractiveMarker::new(position, dimensions, interactive_type);
                interactive_markers.push(interactive_marker);
            }
        }
    }

    interactive_markers
}

pub fn create_interactive_type(properties: &HashMap<String, PropertyValue>) -> InteractiveType {
    let property_value = properties.get("Transition");
    // We assume that there is only one property on a marker
    if let Some(PropertyValue::StringValue(property_string)) = property_value {
        InteractiveType::Transition(PathBuf::from(property_string))
    } else {
        panic!("create_interactive_type: Marker Type is invalid")
    }
}

pub fn flip_interactives_on_y_axis(
    markers: Vec<InteractiveMarker>,
    map_size_in_px: PxDimensions,
    map_grid_dimensions: GridDimensions,
) -> Vec<InteractiveMarker> {
    let mut y_flipped_markers = Vec::new();
    let map_height = map_size_in_px.get_height();
    let tile_height = map_height / map_grid_dimensions.get_rows() as usize;

    for marker in markers {
        let marker_xyzcords = marker.get_position();

        let flipped_ycord = XyzCords::new(
            marker_xyzcords.get_x(),
            flip_y_axis(map_height, marker_xyzcords.get_y() as f32, tile_height) as usize,
            marker_xyzcords.get_z(),
        );

        let flipped_marker = InteractiveMarker::new(
            flipped_ycord,
            marker.get_dimensions(),
            marker.get_interactive_type(),
        );

        y_flipped_markers.push(flipped_marker);
    }

    y_flipped_markers
}
