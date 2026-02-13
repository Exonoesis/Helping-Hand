use bevy::ecs::component::Component;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;
use tiled::ObjectShape;

use crate::map::interactions::map_changing::load_tiled_map;
use crate::map::{is_object_layer, GridCords2D};

#[derive(Clone)]
pub struct SceneNode {
    id: String,
    scene: Scene,
}

impl SceneNode {
    pub fn make_scene_node(id: String, scene: Scene) -> Self {
        SceneNode { id, scene }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_scene(&self) -> &Scene {
        &self.scene
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    title: String,
    scene_type: SceneType,
    scene_contents: SceneContents,
}
impl Scene {
    pub fn make_scene(title: String, scene_type: SceneType, scene_contents: SceneContents) -> Self {
        Scene {
            title,
            scene_type,
            scene_contents,
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_scene_contents(&self) -> &SceneContents {
        &self.scene_contents
    }

    pub fn get_scene_type(&self) -> &SceneType {
        &self.scene_type
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneContents {
    ImageCutscene(PathBuf),
    MapCutscene(PathBuf, Vec<MapAction>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapAction {
    map_instructions: Vec<MapInstruction>,
}
impl MapAction {
    pub fn get_instructions(&self) -> &Vec<MapInstruction> {
        &self.map_instructions
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MapInstruction {
    Wait(Duration),
    Place(Character, MapLocation),
    Move(Character, MapPath),
    Loop(Character, MapPath),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Character {
    name: String,
}

impl Character {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapLocation {
    name: String,
    cords: GridCords2D,
}

impl MapLocation {
    pub fn new(name: String) -> Self {
        // MapLocation's coordinates are set by another source
        // this default should be replaced using the setter
        let default_cords = GridCords2D::new(0, 0);

        Self {
            name,
            cords: default_cords,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_cords(&self) -> &GridCords2D {
        &self.cords
    }

    pub fn set_cords(&mut self, new_cords: GridCords2D) {
        self.cords = new_cords
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapPath {
    name: String,
    map_path: Vec<GridCords2D>,
}

impl MapPath {
    pub fn new(name: String) -> Self {
        // MapPath's map_path is set by another source
        // this default should be replaced using the setter
        let default_map_path: Vec<GridCords2D> = Vec::new();

        Self {
            name,
            map_path: default_map_path,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &Vec<GridCords2D> {
        &self.map_path
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneType {
    ImageCutscene,
    MapCutscene,
}

impl SceneContents {
    pub fn get_image_path(&self) -> &PathBuf {
        if let SceneContents::ImageCutscene(path) = self {
            return path;
        }

        panic!("get_image_path: This was called on a Scene that isn't an Image Cutscene.");
    }

    pub fn get_map_path(&self) -> &PathBuf {
        if let SceneContents::MapCutscene(path, _) = self {
            return path;
        }

        panic!("get_map_path: This was called on a Scene that isn't an Map Cutscene.");
    }

    pub fn get_map_actions(&self) -> &Vec<MapAction> {
        if let SceneContents::MapCutscene(_, actions) = self {
            return actions;
        }

        panic!("get_map_actions: This was called on a Scene that isn't an Map Cutscene.");
    }

    pub fn parse_from(
        arcweave_act_json: &Value,
        scene_type: &SceneType,
        scene_id: &String,
    ) -> SceneContents {
        match scene_type {
            SceneType::ImageCutscene => {
                let image_id = get_scene_image_id(&arcweave_act_json, &scene_id);
                let image_name = get_image_from_id(&arcweave_act_json, image_id);
                let image_path = PathBuf::from(image_name);

                SceneContents::ImageCutscene(image_path)
            }
            SceneType::MapCutscene => {
                let map_path = get_map_path_from_id(&arcweave_act_json, &scene_id);

                let incomplete_map_actions = get_map_actions_from_id(&arcweave_act_json, &scene_id);

                let tiled_map = load_tiled_map(map_path.clone());
                let scene_name = get_title_from_id(&arcweave_act_json, scene_id);

                let finalized_map_actions =
                    get_map_actions_from_map(incomplete_map_actions, tiled_map, scene_name);

                SceneContents::MapCutscene(map_path, finalized_map_actions)
            }
        }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Act {
    scenes: Vec<Scene>,
    current_scene_idx: usize,
    scene_locations: HashMap<String, usize>,
    scene_connections: Vec<Vec<usize>>,
}

impl Act {
    pub fn new() -> Self {
        let scenes = Vec::new();
        let current_scene_idx = 0;
        let scene_locations = HashMap::new();
        let scene_connections = Vec::new();

        Self {
            scenes,
            current_scene_idx,
            scene_locations,
            scene_connections,
        }
    }

    pub fn get_current_scene(&self) -> &Scene {
        &self.scenes.get(self.current_scene_idx).expect(
            &format!(
                "get_current_scene: Scene {} does not exist in scenes. Did you call move_to_next_scene too many times?",
                self.current_scene_idx,
            )
        )
    }

    pub fn get_scene_by_title(&self, title: &String) -> &Scene {
        let found_scene_idx = self.scene_locations.get(title);
        let scene_index = found_scene_idx.expect(&format!(
            "get_scene: Scene with title '{}' not found",
            title
        ));

        &self.scenes[*scene_index]
    }

    pub fn get_scene_idx(&self, scene_to_find: &Scene) -> usize {
        let scene_location = self.scene_locations[&scene_to_find.get_title()];
        scene_location
    }

    pub fn get_scene_connections(&self, scene_to_check: &Scene) -> Vec<&Scene> {
        let checked_scene_idx = self.get_scene_idx(scene_to_check);

        let mut found_scene_connections = Vec::new();

        let checked_scene_connections = &self.scene_connections[checked_scene_idx];
        for connected_scene_idx in checked_scene_connections {
            let found_scene = &self.scenes[*connected_scene_idx];
            found_scene_connections.push(found_scene);
        }

        found_scene_connections
    }

    pub fn add_scene(&mut self, scene: Scene) {
        let scene_title = scene.get_title();

        if self.scene_locations.contains_key(&scene_title) {
            return;
        }

        let index_to_add_at = self.scenes.len();
        self.scene_locations.insert(scene_title, index_to_add_at);

        self.scene_connections.push(Vec::new());

        self.scenes.push(scene);
    }

    pub fn add_scene_connection(&mut self, first_scene: &Scene, second_scene: &Scene) {
        let first_scene_location = self.get_scene_idx(first_scene);
        let second_scene_location = self.get_scene_idx(second_scene);

        self.scene_connections[first_scene_location].push(second_scene_location);
    }

    pub fn move_to_next_scene(&mut self) {
        // TODO: Dynamically change scenes via scene connections + user input
        self.current_scene_idx += 1
    }

    pub fn has_more_scenes(&self) -> bool {
        // TODO: Should check if the current scene has any valid connections,
        // a node with no connections being a dead end thus the end of the act
        self.current_scene_idx < self.scenes.len() - 1
    }
}

/// Converts an arcweave file into a list of Scenes
pub fn read_act_from(act_file: PathBuf) -> Act {
    let mut read_act = Act::new();

    // Create serde_json
    let arcweave_act_json = load_json_file(act_file);

    // Make the first scene in the act
    let starting_scene_name = String::from("startingElement");
    let starting_scene = create_starting_scene(starting_scene_name, &arcweave_act_json);

    // Loop to add all scenes to the act
    // TODO:
    // We will need to track node visits to prevent
    // infinite loops once we have bi-directional edges
    let mut scenes_to_investigate = vec![starting_scene];
    while let Some(current_scene_node) = scenes_to_investigate.pop() {
        read_act.add_scene(current_scene_node.get_scene().clone());

        let next_scenes = create_connected_scenes(&current_scene_node, &arcweave_act_json);
        for next_scene in next_scenes {
            read_act.add_scene(next_scene.get_scene().clone());
            read_act.add_scene_connection(current_scene_node.get_scene(), next_scene.get_scene());
            scenes_to_investigate.push(next_scene);
        }
    }

    read_act
}

fn get_string_from_json_value(json_value: &Value) -> String {
    json_value
        .as_str()
        .expect("Unable to convert value to string.")
        .to_string()
}

fn get_vec_from_json_value(json_value: &Value) -> Vec<Value> {
    json_value
        .as_array()
        .expect("Unable to convert value to array.")
        .clone()
}

/// Modified version of from_reader example of serde_json
fn load_json_file(file_path: PathBuf) -> Value {
    let file = File::open(file_path).expect("load_json_file: Unable to open file");
    let reader = BufReader::new(file);

    let json_value = serde_json::from_reader(reader)
        .expect("load_json_file: Unable to parse JSON file passed in.");

    json_value
}

/// Gets an Arcweave nodes image id
fn get_scene_image_id(act: &Value, id: &String) -> String {
    let image_id_value = act
        .get("elements")
        .and_then(|elements| elements.get(&id))
        .and_then(|element| element.get("assets"))
        .and_then(|assets| assets.get("cover"))
        .and_then(|cover| cover.get("id"))
        .expect(&format!(
            "get_scene_image_id: Unable to get scene image id for item {}",
            id
        ));

    get_string_from_json_value(image_id_value)
}

/// Gets an Arcweave nodes title
fn get_title_from_id(act: &Value, id: &String) -> String {
    let title_value = act
        .get("elements")
        .and_then(|elements| elements.get(&id))
        .and_then(|element| element.get("title"))
        .expect(&format!(
            "get_title_from_id: Unable to get scene title for item {}",
            id
        ));

    let title = get_string_from_json_value(title_value);

    // These titles have html tags which need to be stripped
    strip_html_tags_simple(title)
}

/// Gets an Arcweave nodes image name | ex. Image1.png
fn get_image_from_id(act: &Value, id: String) -> String {
    let image_value = act
        .get("assets")
        .and_then(|assets| assets.get(&id))
        .and_then(|name| name.get("name"))
        .expect(&format!(
            "get_image_from_id: Unable to get image id for item {}",
            id
        ));

    get_string_from_json_value(image_value)
}

/// Gets an Arcweave nodes type name
fn get_scene_type_from_id(act: &Value, id: &String) -> SceneType {
    // Array(Vec<Value>)
    let components_list = act
        .get("elements")
        .and_then(|elements| elements.get(&id))
        .and_then(|componenets| componenets.get("components"))
        .expect(&format!(
            "get_scene_type_from_id: Unable to get components list for item {}",
            id
        ));

    let component_id = components_list.as_array().unwrap().first().unwrap();

    let id_string = get_string_from_json_value(&component_id);

    let component_name = act
        .get("components")
        .and_then(|component| component.get(id_string))
        .and_then(|name| name.get("name"))
        .expect(&format!(
            "get_scene_type_from_id: Unable to get component name for item {}",
            id
        ));

    let type_name = get_string_from_json_value(component_name);

    match type_name.as_str() {
        "Image Cutscene" => return SceneType::ImageCutscene,
        "Map Cutscene" => return SceneType::MapCutscene,
        _ => panic!(
            "get_scene_type_from_id: Unrecognized scene type found: {}",
            type_name
        ),
    }
}

fn get_map_path_from_id(act: &Value, id: &String) -> PathBuf {
    let content_value = act
        .get("elements")
        .and_then(|elements| elements.get(&id))
        .and_then(|content| content.get("content"))
        .expect(&format!(
            "get_map_path_from_id: Unable to get content for item {}",
            id
        ));

    let content_string = get_string_from_json_value(content_value);
    let regex = Regex::new(r#"data-id=\"([0-9a-f-]+)\""#).unwrap();

    let map_component_id = regex
        .captures(&content_string)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap();

    let map_component_attributes_list = act
        .get("components")
        .and_then(|component| component.get(map_component_id))
        .and_then(|attributes| attributes.get("attributes"))
        .expect(&format!(
            "get_map_path_from_id: Unable to get component attribute id for item {}",
            id
        ));

    let map_component_attribute = map_component_attributes_list
        .as_array()
        .unwrap()
        .first()
        .unwrap();

    let map_path_id = get_string_from_json_value(map_component_attribute);

    let map_path_value = act
        .get("attributes")
        .and_then(|attribute| attribute.get(map_path_id))
        .and_then(|value| value.get("value"))
        .and_then(|data| data.get("data"))
        .expect(&format!(
            "get_map_path_from_id: Unable to get attribute data for item {}",
            id
        ));

    let map_path_name = get_string_from_json_value(map_path_value);

    // Need to prefix folder path
    let folder_path = PathBuf::from("assets/map/");
    let full_path = folder_path.join(map_path_name);

    PathBuf::from(full_path)
}

/// This is called first and extracts all map actions from an Arcweave File
/// without information from the map such as coordinates
fn get_map_actions_from_id(act: &Value, id: &String) -> Vec<MapAction> {
    let content_value = act
        .get("elements")
        .and_then(|elements| elements.get(&id))
        .and_then(|content| content.get("content"))
        .expect(&format!(
            "get_map_actions_from_id: Unable to get content for item {}",
            id
        ));

    let content_string = get_string_from_json_value(content_value);

    // Strip HTML + other noise
    let cleaned_map_cutscene_content = strip_html_for_map_actions(content_string.as_str());

    // Send cleaned content to parse_map_actions
    let map_actions = parse_map_actions(cleaned_map_cutscene_content.as_str());

    map_actions
}

/// This is called second and extracts all map information from a Tiled Map
/// such as coordinates, returning a finalized collection of MapActions
fn get_map_actions_from_map(
    incomplete_map_actions: Vec<MapAction>,
    tiled_map: tiled::Map,
    scene_name: String,
) -> Vec<MapAction> {
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;
    let mut complete_map_actions = Vec::new();

    for z in 0..tiled_map.layers().len() {
        let is_object_layer = is_object_layer(&tiled_map, z);
        if !is_object_layer {
            continue;
        }

        let layer = tiled_map
            .get_layer(z)
            .expect("get_map_actions_from_map: Failed to unwrap layer.");
        if layer.name != scene_name {
            continue;
        }

        let object_layer = layer
            .as_object_layer()
            .expect("get_map_actions_from_map: Failed to unwrap layer as object layer.");

        let actions_list = incomplete_map_actions.clone();

        for action in actions_list {
            let mut new_instructions = Vec::new();
            for instruction in action.get_instructions() {
                match instruction {
                    MapInstruction::Place(character, map_location) => {
                        let placement_object = object_layer
                            .objects()
                            .find(|object| {
                                object.user_type == "Placement"
                                    && object.name == *map_location.get_name()
                            })
                            .expect(&format!(
                                "get_map_actions_from_map: No placement object with name {} found",
                                map_location.get_name(),
                            ));

                        let new_x = (placement_object.x as u32 / tile_width) as usize;
                        let new_y = (placement_object.y as u32 / tile_height) as usize;

                        let new_cords = GridCords2D::new(new_x, new_y);

                        let new_map_location = MapLocation {
                            name: map_location.get_name().clone(),
                            cords: new_cords,
                        };
                        new_instructions
                            .push(MapInstruction::Place(character.clone(), new_map_location));
                    }
                    MapInstruction::Move(character, map_path) => {
                        let move_path_object = object_layer
                            .objects()
                            .find(|object| {
                                object.user_type == "Path" && object.name == *map_path.get_name()
                            })
                            .expect(&format!(
                                "get_map_actions_from_map: No placement object with name {} found",
                                map_path.get_name(),
                            ));

                        if let ObjectShape::Polyline { points } = &move_path_object.shape {
                            let new_map_path = get_path_from_points(move_path_object, points);

                            let new_map_path = MapPath {
                                name: map_path.get_name().clone(),
                                map_path: new_map_path,
                            };
                            new_instructions
                                .push(MapInstruction::Move(character.clone(), new_map_path));
                        }
                    }
                    MapInstruction::Loop(character, map_path) => {
                        // TODO
                    }
                    MapInstruction::Wait(_) => {}
                }
            }
            complete_map_actions.push(MapAction {
                map_instructions: new_instructions,
            });
        }
    }
    complete_map_actions
}

// TODO: [Math Helper: get_path_from_points]
// [Number of line segments to calculate is -1 the number of polyline points]
// [ex. <1,2,3,4,5>]
//
// line_seg_num = 0;
// for line_seg_num < polyline points
// {
// point 1 = segnum
// point 2 = segnum + 1
//
// grid point 1 = ((origin x + point 1 x), (origin y + point 1 y)) trunc both
// grid point 2 = ((origin x + point 2 x), (origin y + poiunt 2 y)) trunc both
//
// Determine direction
// [Handle x-axis movements (up, down)]
// [Handle y-axis movements (left, right)]
// [We have a direction enum? Can reuse?]
//
// [Match on direction and have math loops in each branch]
// {
// Add first tile into path_tile Vec
// Loop the difference by going in direction (mathematically)
// Place tiles in path_tile Vec
// Add last tile into path_tile Vec
// }
//
// segnum++
// }
fn get_path_from_points(
    move_path_object: tiled::Object<'_>,
    points: &[(f32, f32)],
) -> Vec<GridCords2D> {
    let origin_x = move_path_object.x;
    let origin_y = move_path_object.y;
    let mut final_path = Vec::new();

    let from_point = GridCords2D::new(
        (origin_x + points[0].0) as usize,
        (origin_y + points[0].1) as usize,
    );

    let to_point = GridCords2D::new(
        (origin_x + points[1].0) as usize,
        (origin_y + points[1].1) as usize,
    );

    // calculate distance between two
    // make a compare fn in GridCords2D?
    // returns X or Y and pos or neg num?

    final_path.push(from_point);

    final_path
}

fn strip_html_for_map_actions(input: &str) -> String {
    let remove_simple_tags = input
        .replace("<p>", "")
        .replace("</p>", "")
        .replace("</span>", "");

    let remove_span_tags = Regex::new(r#"<span[^>]*>"#)
        .unwrap()
        .replace_all(&remove_simple_tags, "")
        .to_string();

    let stripped_before_first_bracket = remove_span_tags
        .split_once('[')
        .map(|(_before, after)| format!("[{}", after))
        .unwrap_or(remove_span_tags);

    let remove_outer_whitespace = Regex::new(r"\]\s*\[")
        .unwrap()
        .replace_all(&stripped_before_first_bracket, "][")
        .to_string();

    remove_outer_whitespace
}

/// Takes a batch of map actions, each enclosed within brackets,
/// and converts them into proper MapActions
fn parse_map_actions(map_cutscene_contents: &str) -> Vec<MapAction> {
    let mut collected_map_actions: Vec<MapAction> = Vec::new();

    let trimmed_map_cutscene_contents = map_cutscene_contents.trim_matches(['[', ']']);
    let split_map_cutscene_contents: Vec<&str> =
        trimmed_map_cutscene_contents.split("][").collect();

    for batch in split_map_cutscene_contents {
        let cleaned_batch = batch.replace("[", "").replace("]", "");
        let map_action = MapAction {
            map_instructions: parse_map_instructions(cleaned_batch.as_str()),
        };
        collected_map_actions.push(map_action);
    }

    collected_map_actions
}

/// Takes a batch of comma separated map instructions and
/// converts them into their MapInstruction equivalent
fn parse_map_instructions(map_instruction_batch: &str) -> Vec<MapInstruction> {
    let mut parsed_map_instructions: Vec<MapInstruction> = Vec::new();

    let split_map_instruction_batch: Vec<&str> = map_instruction_batch.split(',').collect();

    for single_map_instruction in split_map_instruction_batch {
        let split_map_instruction: Vec<&str> = single_map_instruction.split_whitespace().collect();

        let instruction_action = split_map_instruction[1];

        match instruction_action {
            "@" => {
                parsed_map_instructions.push(MapInstruction::Place(
                    Character::new(split_map_instruction[0].to_string()),
                    MapLocation::new(split_map_instruction[2].to_string()),
                ));
                continue;
            }
            ">" => {
                parsed_map_instructions.push(MapInstruction::Move(
                    Character::new(split_map_instruction[0].to_string()),
                    MapPath::new(split_map_instruction[2].to_string()),
                ));
                continue;
            }
            "<->" => {
                parsed_map_instructions.push(MapInstruction::Loop(
                    Character::new(split_map_instruction[0].to_string()),
                    MapPath::new(split_map_instruction[2].to_string()),
                ));
                continue;
            }
            _ => {}
        }

        let special_instruction = split_map_instruction[0];
        let instruction_duration = split_map_instruction[1];

        match special_instruction {
            "Wait" => {
                let duration = str_to_duration(instruction_duration);
                parsed_map_instructions.push(MapInstruction::Wait(duration));
                continue;
            }
            _ => panic!(
                "parse_map_instructions: Unrecognized instruction found: {}",
                special_instruction
            ),
        }
    }

    parsed_map_instructions
}

/// Takes an str in the format of: [number]s and returns a duration in seconds
/// Example: "16s" would return a Duration of 16 seconds
fn str_to_duration(duration_str: &str) -> Duration {
    let trimmed_duration_str = duration_str.trim_end_matches("s").parse::<u64>().unwrap();
    let duration = Duration::from_secs(trimmed_duration_str);

    duration
}

/// Gets an Arcweave nodes list of outputs
fn get_list_of_scene_connections(
    arcweave_act_json: &Value,
    current_scene_id: &String,
) -> Vec<Value> {
    // Get list of connections for this scene
    let scene_connections = arcweave_act_json
        .get("elements")
        .and_then(|elements| elements.get(&current_scene_id))
        .and_then(|element| element.get("outputs"))
        .expect(&format!(
            "create_connected_scenes: Unable to get scene outputs for item {}",
            current_scene_id
        ));
    get_vec_from_json_value(scene_connections)
}

/// Gets an Arcweave connection target
fn get_target_id(arcweave_act_json: &Value, connection_id: String) -> String {
    let target_scene_id = arcweave_act_json
        .get("connections")
        .and_then(|connections| connections.get(&connection_id))
        .and_then(|connection| connection.get("targetid"))
        .expect(&format!(
            "create_connected_scenes: Unable to get connection targetid for item {}",
            connection_id
        ));

    get_string_from_json_value(target_scene_id)
}

/// Creates a SceneNode from the starting scene name
fn create_starting_scene(scene_name: String, arcweave_act_json: &Value) -> SceneNode {
    let scene_value = arcweave_act_json.get(scene_name).unwrap();
    let id = get_string_from_json_value(scene_value);

    create_scene_from_id(id, arcweave_act_json)
}

/// Creates a SceneNode from a given id
fn create_scene_from_id(id: String, arcweave_act_json: &Value) -> SceneNode {
    let title = get_title_from_id(&arcweave_act_json, &id);
    let scene_type = get_scene_type_from_id(&arcweave_act_json, &id);
    let scene_contents = SceneContents::parse_from(arcweave_act_json, &scene_type, &id);

    let scene = Scene::make_scene(title, scene_type, scene_contents);
    SceneNode::make_scene_node(id, scene)
}

/// Returns a list of SceneNodes connected to a given SceneNode
fn create_connected_scenes(
    current_scene_node: &SceneNode,
    arcweave_act_json: &Value,
) -> Vec<SceneNode> {
    let mut connected_scenes = Vec::new();

    let current_scene_id = current_scene_node.get_id();

    let scene_connection_collection =
        get_list_of_scene_connections(arcweave_act_json, current_scene_id);

    // For each connection, get the scene it connects to and add it to the final list
    for connection in scene_connection_collection {
        let connection_id = get_string_from_json_value(&connection);

        let target_scene_id = get_target_id(arcweave_act_json, connection_id);
        let connected_scene = create_scene_from_id(target_scene_id, arcweave_act_json);

        connected_scenes.push(connected_scene);
    }

    connected_scenes
}

/// Removes HTML tags added by Arcweave | ex. <p>text<\/p>
pub fn strip_html_tags_simple(line: String) -> String {
    // Create a regex to match HTML tags
    let regex = Regex::new(r"<[^>]*>").unwrap();

    // Replace HTML tags with the empty string
    let cleaned_line = regex.replace_all(&line, "");

    // Turn the line back into a String
    cleaned_line.to_string()
}

#[cfg(test)]
mod tests {
    // Import/use any function outside of this test module.
    use super::*;

    // This is your brain on unit testing.
    #[test]
    fn str_to_duration_test() {
        let duration_string = "10s";

        let expected_duration = Duration::from_secs(10);
        let actual_duration = str_to_duration(duration_string);

        assert_eq!(expected_duration, actual_duration);
    }

    #[test]
    fn parse_map_instructions_test() {
        let instruction_string = "Player @ Place, OtherPlayer > OverThere";

        let instruction_vector = parse_map_instructions(instruction_string);

        let actual_first_instruction = instruction_vector[0].clone();
        let actual_second_instruction = instruction_vector[1].clone();

        let expected_first_instruction = MapInstruction::Place(
            Character::new("Player".to_string()),
            MapLocation::new("Place".to_string()),
        );

        let expected_second_instruction = MapInstruction::Move(
            Character::new("OtherPlayer".to_string()),
            MapPath::new("OverThere".to_string()),
        );

        assert_eq!(
            expected_first_instruction, actual_first_instruction,
            "First instruction mismatch"
        );
        assert_eq!(
            expected_second_instruction, actual_second_instruction,
            "Second instruction mismatch"
        );
    }

    #[test]
    fn parse_map_action_test() {
        let map_action_string =
            "[Player @ Place, OtherPlayer > OverThere][PlayerThree <-> CircleTime]";

        let action_vector = parse_map_actions(map_action_string);

        let actual_first_action = action_vector[0].clone();
        let actual_second_action = action_vector[1].clone();

        let mut first_action_instructions = Vec::new();
        let first_map_action_first_instruction = MapInstruction::Place(
            Character::new("Player".to_string()),
            MapLocation::new("Place".to_string()),
        );

        let first_map_action_second_instruction = MapInstruction::Move(
            Character::new("OtherPlayer".to_string()),
            MapPath::new("OverThere".to_string()),
        );

        first_action_instructions.push(first_map_action_first_instruction);
        first_action_instructions.push(first_map_action_second_instruction);

        let mut second_action_instructions = Vec::new();
        let second_map_action_first_instruction = MapInstruction::Loop(
            Character::new("PlayerThree".to_string()),
            MapPath::new("CircleTime".to_string()),
        );

        second_action_instructions.push(second_map_action_first_instruction);

        let expected_first_action = MapAction {
            map_instructions: first_action_instructions,
        };

        let expected_second_action = MapAction {
            map_instructions: second_action_instructions,
        };

        assert_eq!(
            expected_first_action, actual_first_action,
            "First action mismatch"
        );
        assert_eq!(
            expected_second_action, actual_second_action,
            "Second action mismatch"
        );
    }
}
