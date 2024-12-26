use crate::model::game_documents::GameDocuments;
use crate::model::save::Autosave;
use crate::{
    init_json_data_explicit_save_file, read_config
};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::model::FindById;
use test_case::test_case;

fn get_save_file_path(save_file_name: &str) -> anyhow::Result<PathBuf> {
    let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // get save file path
    let save_file_path = manifest_directory
        .join("resources")
        .join("test")
        .join("savefiles")
        .join("AUTOSAVE.json");

    println!("AUTOSAVE file path: {:?}", save_file_path.to_string_lossy());

    Ok(save_file_path)
}

fn load_save_file(save_file_name: &str) -> anyhow::Result<Autosave> {
    let save_file_path = get_save_file_path(save_file_name)?;

    // Open the file in read-only mode with buffer.
    let file = File::open(save_file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Autosave`.
    let u: Autosave = serde_json::from_reader(reader)?;

    Ok(u)
}

fn get_game_documents(save_file_name: &str) -> anyhow::Result<Arc<RwLock<GameDocuments>>> {
    let bhcontent_core_path = read_config()?;

    let save_file_path = &get_save_file_path(save_file_name)?;

    let game_documents_arc =
        init_json_data_explicit_save_file(&bhcontent_core_path, save_file_path)?;

    Ok(game_documents_arc)
}

#[test]
fn load_save_file_test() {
    let save = load_save_file("AUTOSAVE.json").unwrap();
}

#[test_case("black.sapphire.wash", "AUTOSAVE.json" => panics "Item with ID \"black.sapphire.wash\" has not yet been manifested")]
fn check_if_item_already_crafted(item_id: &str, save_file_name: &str) {
    let gdc = get_game_documents(save_file_name).expect("Failed to get game documents");

    let game_documents = gdc.read().expect("Failed to read game documents");

    let item = game_documents
        .aspected_items
        .get_memory_from_id(item_id)
        .expect("Failed to get memory string from id");
    println!("Item: {:?}", item.label);

    let item_manifested = game_documents
        .check_if_item_manifested_fuzzy(item)
        .expect("Failed to check if item has been manifested");
    println!("Item manifested: {:?}", item_manifested);
    assert_eq!(
        item_manifested, true,
        "Item with ID \"{}\" has not yet been manifested",
        item_id
    );
}

#[test_case(
    "craft.scholar.desires.dissolutions_flower_rubywise.ruin_grail",
    "AUTOSAVE.json"
)]
fn check_if_recipe_unlocked(item_id: &str, save_file_name: &str) {
    let gdc = get_game_documents(save_file_name).expect("Failed to get game documents");

    let game_documents = gdc.read().expect("Failed to read game documents");

    println!("Attempting to get memory with ID \"{}\"", item_id);
    let item = game_documents
        .recipes
        .get_collection()
        .find_by_id_case_insensitive(item_id)
        .expect("Failed to get recipe with ID");
    println!("Item: {:?}", item.label);

    let recipe_unlocked = game_documents
        .check_if_recipe_unlocked(item)
        .expect("Failed to check if recipe has been unlocked");
    println!("Recipe unlocked: {:?}", recipe_unlocked);
    assert_eq!(
        recipe_unlocked, true,
        "Recipe with ID \"{}\" has not yet been unlocked",
        item_id
    );
}
