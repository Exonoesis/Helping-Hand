//use bevy::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

use std::path::PathBuf;

// TODO: Call this SceneContents and add to new Scene struct
#[derive(Debug, PartialEq)]
pub enum Scene {
    ImageCutscene(PathBuf),
}

#[derive(Debug)]
pub struct Act {
    scenes: Vec<Scene>,
}

impl Act {
    pub fn new() -> Self {
        let scenes = Vec::new();

        Self { scenes }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn get_scene(&self, index: usize) -> &Scene {
        &self.scenes[index]
    }
}

// Converts an arcweave file into a list of Scenes
pub fn read_act_from(act_file: PathBuf) -> Act {
    let mut read_act = Act::new();

    // Create serde_json
    let arcweave_act = load_json_file(act_file);

    /*
    let starting_scene_id = "startingElement"
    let starting_scene = read_scene(starting_scene_id, arcweave_act_json)
    scenes_to_investigate = vec![starting_scene]
    while there are scenes to investigate:
        # This is a Node, consisting of an ID, and a Scene, with the following technical bits:
        - The ID of the Scene (Thanks Arcweave, unless there's a way around this.)
        - The data, which is the Scene below ---V

        # This is a Scene
        - The title of the Scene (Node ID),
        - The contents of the Scene (ImageCutscene has a Path, etc) (Node data).
        let current_scene_node = scenes_to_investigate.pop()
        read_act.add_scene(current_scene_node.get_scene())

        let next_scenes = get_connected_scenes(current_scene_node, arcweave_act_json)
        for next_scene in next_scenes
            read_act.add_scene_connection(current_scene.get_scene(), next_scene.get_scene())

            scenes_to_investigate.push(next_scene)
    */

    // Get the id of the first scene in the act
    let starting_node_name = String::from("startingElement");
    let starting_node_value = arcweave_act.get(starting_node_name).unwrap();
    let first_scene_name = get_string_from_json_value(starting_node_value);

    // Look up scene and access it's image id
    let image_id_name = get_scene_image_id(&arcweave_act, first_scene_name);

    // Look up image by id
    let image_name = get_image_from_id(&arcweave_act, image_id_name);

    let image_path = PathBuf::from(image_name);
    read_act.add_scene(Scene::ImageCutscene(image_path));

    read_act
}

fn get_string_from_json_value(json_value: &Value) -> String {
    json_value
        .as_str()
        .expect("Unable to convert value to string.")
        .to_string()
}

// Modified version of from_reader example of serde_json
fn load_json_file(file_path: PathBuf) -> Value {
    let file = File::open(file_path).expect("load_json_file: Unable to open file");
    let reader = BufReader::new(file);

    let json_value = serde_json::from_reader(reader)
        .expect("load_json_file: Unable to parse JSON file passed in.");

    json_value
}

fn get_scene_image_id(act: &Value, id: String) -> String {
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
