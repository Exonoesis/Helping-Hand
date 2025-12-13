use std::{ffi::OsString, path::PathBuf};

pub mod player;
pub mod render;

use bevy::prelude::*;

use tiled::{Map, Object};

pub mod interactions;
pub mod movement;

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
        let mut tiled_tiles = get_environment_tiles(&tiled_map);
        let found_player = get_player(&tiled_map);

        // There exist map cutscenes. Some map cutscenes can not have a
        // player on it, such as showing something that happened in the past
        // with unrelated characters.
        if let Some(player) = found_player {
            tiled_tiles.push(player);
        }

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

#[derive(Debug)]
pub struct Tile {
    tile_dimensions: PxDimensions,
    px_cords: PxCords,
    grid_cords: GridCords,
    tile_texture: Option<TileTexture>,
    //layer_number: usize,
    tile_type: TileType,
}

impl Tile {
    pub fn new(
        tile_dimensions: PxDimensions,
        px_cords: PxCords,
        grid_cords: GridCords,
        tile_texture: Option<TileTexture>,
        //layer_number: usize,
        tile_type: TileType,
    ) -> Tile {
        Tile {
            tile_dimensions,
            px_cords,
            grid_cords,
            tile_texture,
            //layer_number,
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

    pub fn get_grid_coordinates(&self) -> &GridCords {
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

fn get_environment_tiles(tiled_map: &Map) -> Vec<Tile> {
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
                let px_cords = PxCords::new_u32(x * tile_width, y * tile_height, z);
                let grid_cords = GridCords::new_u32(x, y, z);
                let tile_texture = get_environmental_tile_texture(&tiled_map, x, y, z);
                //let layer_number = z;
                let tile_type = get_environmental_tile_type(&tiled_map, x, y, z);

                let tile = Tile::new(
                    tile_dimensions,
                    px_cords,
                    grid_cords,
                    tile_texture,
                    //layer_number,
                    tile_type,
                );
                tiles.push(tile);
            }
        }
    }

    tiles
}

/// Returns a player found on the map from the Interaction layer if it exists.
/// Returns None otherwise.
fn get_player(tiled_map: &Map) -> Option<Tile> {
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;

    let tile_dimensions = PxDimensions::new(tile_width, tile_height);

    for z in 0..tiled_map.layers().len() {
        let is_object_layer = is_object_layer(&tiled_map, z);
        if !is_object_layer {
            continue;
        }

        let layer = tiled_map.get_layer(z).unwrap();
        if layer.name != "Interaction" {
            continue;
        }

        let object_layer = tiled_map.get_layer(z).unwrap().as_object_layer().unwrap();

        for object in object_layer.objects() {
            if object.user_type == "Player" {
                let x = object.x as u32;
                let y = object.y as u32;
                let px_cords = PxCords::new_u32(x, y - tile_height, z);
                let grid_cords = GridCords::new_u32(x / tile_width, (y / tile_height) - 1, z);
                let tile_texture = Some(get_player_tile_texture(&object));
                //let layer_number = z;
                let tile_type = TileType::Player;

                return Some(Tile::new(
                    tile_dimensions,
                    px_cords,
                    grid_cords,
                    tile_texture,
                    //layer_number,
                    tile_type,
                ));
            }
        }
    }

    None
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

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Normal,
    Player,
    Collision,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PxCords {
    px_x: usize,
    px_y: usize,
    px_z: usize,
}

impl PxCords {
    pub fn new(px_x: usize, px_y: usize, px_z: usize) -> Self {
        PxCords { px_x, px_y, px_z }
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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GridCords {
    grid_x: usize,
    grid_y: usize,
    grid_z: usize,
}

impl GridCords {
    pub fn new(grid_x: usize, grid_y: usize, grid_z: usize) -> Self {
        GridCords {
            grid_x,
            grid_y,
            grid_z,
        }
    }

    pub fn new_u32(grid_x: u32, grid_y: u32, grid_z: usize) -> Self {
        Self::new(grid_x as usize, grid_y as usize, grid_z)
    }

    pub fn get_x(&self) -> usize {
        self.grid_x
    }

    pub fn get_y(&self) -> usize {
        self.grid_y
    }

    pub fn get_z(&self) -> usize {
        self.grid_z
    }
}

#[derive(Debug)]
pub struct TileTexture {
    spritesheet: PathBuf,
    sprite_index: usize,
    spritesheet_dimensions: PxDimensions,
}

fn is_tile_layer(tiled_map: &Map, idx: usize) -> bool {
    let found_tile_layer = tiled_map.get_layer(idx).unwrap().as_tile_layer();

    found_tile_layer.is_some()
}

fn is_object_layer(tiled_map: &Map, idx: usize) -> bool {
    let found_object_layer = tiled_map.get_layer(idx).unwrap().as_object_layer();

    found_object_layer.is_some()
}

fn get_player_tile_texture(object: &Object) -> TileTexture {
    let tile = object
        .get_tile()
        .expect("get_player_tile_texture: Player does not have a tile.");
    let sprite_index = tile.id() as usize;
    let spritesheet = tile.get_tileset().image.clone().unwrap().source;
    let spritesheet_px_width = tile.get_tileset().image.as_ref().unwrap().width as u32;
    let spritesheet_px_height = tile.get_tileset().image.as_ref().unwrap().height as u32;

    TileTexture {
        sprite_index,
        spritesheet,
        spritesheet_dimensions: PxDimensions::new(spritesheet_px_width, spritesheet_px_height),
    }
}

fn get_environmental_tile_texture(
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

fn get_environmental_tile_type(
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
        "Collision" => TileType::Collision,
        _ => TileType::Normal,
    }
}

pub fn flip_y_axis(map_height: usize, tile_y: f32, tile_height: usize) -> f32 {
    map_height as f32 - tile_y - tile_height as f32
}
