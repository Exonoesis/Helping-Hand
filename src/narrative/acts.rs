use regex::Regex;
use serde_json::Value;
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
    scene_contents: SceneContents,
}
impl Scene {
    pub fn make_scene(title: String, scene_contents: SceneContents) -> Self {
        Scene {
            title,
            scene_contents,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneContents {
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

    // Replace this with a way to look up scene by title
    pub fn get_scene(&self, index: usize) -> &Scene {
        &self.scenes[index]
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

    let mut scenes_to_investigate = vec![starting_scene];

    while let Some(current_scene_node) = scenes_to_investigate.pop() {
        read_act.add_scene(current_scene_node.get_scene().clone());

        let next_scenes = create_connected_scenes(current_scene_node, &arcweave_act_json);
        for next_scene in next_scenes {
            // To-Do: Discuss how to store scene_connections
            // read_act.add_scene_connection(current_scene_node.get_scene(), next_scene.get_scene());

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

fn create_scene_from_id(id: String, arcweave_act_json: &Value) -> SceneNode {
    // Look up scene and access its title
    let title = get_title_from_id(&arcweave_act_json, &id);

    // Look up scene and access its image
    let image_id = get_scene_image_id(&arcweave_act_json, &id);
    let image_name = get_image_from_id(&arcweave_act_json, image_id);
    let image_path = PathBuf::from(image_name);
    let scene_contents = SceneContents::ImageCutscene(image_path);

    // Make a Scene
    let scene = Scene::make_scene(title, scene_contents);

    // Boom, SceneNode
    SceneNode::make_scene_node(id, scene)
}

/// Returns a list of SceneNode's connected to a given SceneNode
fn create_connected_scenes(
    current_scene_node: SceneNode,
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
