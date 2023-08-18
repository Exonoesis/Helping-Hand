use bevy::{
    asset::{AssetIo, AssetIoError, Metadata, ChangeWatcher},
    prelude::*,
    utils::BoxedFuture,
};
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use unicode_segmentation::UnicodeSegmentation;

/// A custom asset io implementation that logs file suggestions for missing files
/// in loading.
pub struct SmartAssetIo(pub Box<dyn AssetIo>);

/// Appends to the results array all files and directories discovered
/// in a Breadth-First Traversal starting from the current_path as root.
fn breadth_first_search(current_path: &Path, results: &mut Vec<PathBuf>) {
    if !current_path.is_dir() {
        return;
    }

    for entry in read_dir(current_path)
        .expect("This directory should exist.")
        .flatten()
    {
        results.push(entry.path());
    }

    for entry in read_dir(current_path)
        .expect("This directory should exist.")
        .flatten()
    {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            breadth_first_search(&entry_path, results);
        }
    }
}

/// Returns the minimum number of edits required to map word1 to word2.
///
/// NOTE: This is a famous Dynamic Programming algorithm. For more information, see
/// this: https://en.wikipedia.org/wiki/Edit_distance
fn edit_distance(word1: &str, word2: &str) -> usize {
    let word1_chars = word1.graphemes(true).collect::<Vec<&str>>();
    let word2_chars = word2.graphemes(true).collect::<Vec<&str>>();

    let mut distances_from_to: Vec<Vec<usize>> =
        vec![vec![0; word2_chars.len() + 1]; word1_chars.len() + 1];

    // I ignored this clippy warning to keep the code
    // as close to the original implementation as possible
    // while keeping it readable.
    #[allow(clippy::needless_range_loop)]
    for i in 0..=word1_chars.len() {
        distances_from_to[i][0] = i;
    }

    for j in 0..=word2_chars.len() {
        distances_from_to[0][j] = j;
    }

    for i in 1..=word1_chars.len() {
        for j in 1..=word2_chars.len() {
            let word1_char = word1_chars[i - 1];
            let word2_char = word2_chars[j - 1];

            if word1_char == word2_char {
                distances_from_to[i][j] = distances_from_to[i - 1][j - 1];
            } else {
                let insertion_op = distances_from_to[i][j - 1] + 1;
                let removal_op = distances_from_to[i - 1][j] + 1;
                let replacement_op = distances_from_to[i - 1][j - 1] + 1;

                let result = insertion_op.min(removal_op).min(replacement_op);
                distances_from_to[i][j] = result;
            }
        }
    }

    distances_from_to[word1_chars.len()][word2_chars.len()]
}

/// Returns an absolute path starting from the root of the file system
/// to the asset directory's entry containing the file or directory of
/// the original_path.
fn to_canonical_asset_path(original_path: &Path) -> PathBuf {
    let root_directory = env!("CARGO_MANIFEST_DIR").to_string();
    let asset_directory = root_directory + "/assets/" + original_path.to_str().unwrap();

    Path::new(&asset_directory).to_path_buf()
}

/// Returns a vector of all known files in the assets folder.
fn get_all_asset_file_paths() -> Vec<PathBuf> {
    let assets_folder_path = to_canonical_asset_path(&PathBuf::new());

    let mut asset_file_paths = Vec::new();
    asset_file_paths.push(assets_folder_path.to_path_buf());
    breadth_first_search(&assets_folder_path, &mut asset_file_paths);

    asset_file_paths
}

/// Returns the path to the file or directory closest in match
/// to the original_path.
fn get_closest_file_path(original_path: &Path) -> PathBuf {
    let asset_file_paths = get_all_asset_file_paths();

    let mut file_difference_scores = Vec::new();
    let mut file_indexes = (0..asset_file_paths.len()).collect::<Vec<usize>>();

    for asset_file_path in &asset_file_paths {
        file_difference_scores.push(edit_distance(
            original_path.to_str().unwrap(),
            asset_file_path.to_str().unwrap(),
        ));
    }

    // The smallest edit distance indicates it's the closest match
    // we're going to get for our educated guess.
    file_indexes
        .sort_by(|&idx1, &idx2| file_difference_scores[idx1].cmp(&file_difference_scores[idx2]));
    asset_file_paths[file_indexes[0]].clone()
}

impl AssetIo for SmartAssetIo {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        let asset_path = to_canonical_asset_path(path);
        if !asset_path.exists() {
            let closest_path = get_closest_file_path(&asset_path);
            error!("load_path: path {:?} does not exist.", asset_path);
            info!("SUGGESTION: Did you mean {:?}?", closest_path);
        }

        self.0.load_path(path)
    }

    fn read_directory(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        self.0.read_directory(path)
    }

    fn watch_path_for_changes(
        &self,
        to_watch: &Path,
        to_reload: Option<PathBuf>,
    ) -> Result<(), AssetIoError> {
        self.0.watch_path_for_changes(to_watch, to_reload)
    }

    fn watch_for_changes(&self, configuration: &ChangeWatcher) -> Result<(), AssetIoError> {
        self.0.watch_for_changes(configuration)
    }

    fn get_metadata(&self, path: &Path) -> Result<Metadata, AssetIoError> {
        self.0.get_metadata(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn can_find_assets_folder() {
        let assets_folder_path = to_canonical_asset_path(&PathBuf::new());

        assert!(assets_folder_path.exists());
    }

    // The .github workflow directory was chosen since it's the least likely
    // to change over-time, making for a great unit testing candidate for something
    // that's supposed to work on any directory as-is.
    #[test]
    fn breadth_first_search_workflow_dir() {
        let mut files_in_dir = Vec::new();

        let root_directory = env!("CARGO_MANIFEST_DIR").to_string();
        let workflows_directory = root_directory + "/.github/";
        let workflow_folder_path = Path::new(&workflows_directory);

        assert!(workflow_folder_path.exists());

        breadth_first_search(workflow_folder_path, &mut files_in_dir);
        assert_eq!(files_in_dir.len(), 4);
    }

    #[test]
    fn extra_letter() {
        let asset_path = to_canonical_asset_path(Path::new("map/hh_worlds.ldtk"));

        assert!(!asset_path.exists());

        let expected_closest_path = to_canonical_asset_path(Path::new("map/hh_world.ldtk"));
        let actual_closest_path = get_closest_file_path(&asset_path);

        assert_eq!(expected_closest_path, actual_closest_path);
    }

    #[test]
    fn wrong_file_extension() {
        let asset_path = to_canonical_asset_path(Path::new("map/hh_world.jpg"));

        assert!(!asset_path.exists());

        let expected_closest_path = to_canonical_asset_path(Path::new("map/hh_world.ldtk"));
        let actual_closest_path = get_closest_file_path(&asset_path);

        assert_eq!(expected_closest_path, actual_closest_path);
    }

    #[test]
    fn wrong_folder_name() {
        let asset_path = to_canonical_asset_path(Path::new("textures/hh_world.ldtk"));

        assert!(!asset_path.exists());

        let expected_closest_path = to_canonical_asset_path(Path::new("map/hh_world.ldtk"));
        let actual_closest_path = get_closest_file_path(&asset_path);

        assert_eq!(expected_closest_path, actual_closest_path);
    }
}
