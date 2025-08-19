use bevy::ecs::component::Component;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

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
    MapCutscene(PathBuf, Vec<MapCommand>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapCommand {}

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
                // TODO
                let map_path = PathBuf::new();
                let scene_commands = get_scene_commands_from_id();

                SceneContents::MapCutscene(map_path, scene_commands)
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
    strip_html_tags(title)
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

// TODO:
fn get_scene_commands_from_id() -> Vec<MapCommand> {
    Vec::new()
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

/// Removes html tags added by Arcweave | ex. <p>text<\/p>
pub fn strip_html_tags(line: String) -> String {
    // Create a regex to match HTML tags
    let regex = Regex::new(r"<[^>]*>").unwrap();

    // Replace HTML tags with the empty string
    let cleaned_line = regex.replace_all(&line, "");

    // Turn the line back into a String
    cleaned_line.to_string()
}
