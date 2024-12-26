use crate::model::save::Autosave;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn load_save_file(save_file_path: &PathBuf) -> anyhow::Result<Autosave> {
    // Open the file in read-only mode with buffer.
    let file = File::open(save_file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Autosave`.
    let u: Autosave = serde_json::from_reader(reader)?;

    Ok(u)
}

#[test]
fn load_save_file_test() {
    let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // load save file
    let save_file = manifest_directory
        .join("resources")
        .join("test")
        .join("savefiles")
        .join("AUTOSAVE.json");
    println!("AUTOSAVE file path: {:?}", save_file.to_string_lossy());
    let save = load_save_file(&save_file).unwrap();
}
