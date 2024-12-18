mod logging;

mod model;
mod utils;

use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use tracing::{debug, error, info, trace, warn};

use encoding_rs::{UTF_16LE, UTF_8};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};

use crate::model::aspected_items::AspectedItems;
use crate::model::aspects::Aspects;
use crate::model::config::Config;
use crate::model::consider_books::ConsiderBooks;
use crate::model::lessons::Lessons;
use crate::model::save::{Autosave, Path};
use crate::model::skills::Skills;
use crate::model::tomes::Tomes;
use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use anyhow::{anyhow, bail, Error};
use clipboard_rs::{Clipboard, ClipboardContext};
use crossbeam_channel::{select, unbounded};
use notify::event::ModifyKind;
use notify::RecursiveMode::Recursive;
use rustyline::history::DefaultHistory;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;
use model::game_documents::GameDocuments;
use utils::save;

static APP_PATH_FULL: &str = "evanjs/weatherfactory-tools/book-of-hours_cli";
static APP_CONFIG_FILE_NAME: &str = "app_config";

#[derive(Debug, Serialize, Deserialize)]
struct Queries {
    name_query: String,
    query_type: QueryType,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone, PartialEq, Eq)]
enum QueryType {
    #[strum(serialize = "skills")]
    Skills,
    #[strum(serialize = "aspects")]
    Aspects,
    #[strum(serialize = "tomes")]
    Tomes,
    #[strum(serialize = "aspected items")]
    AspectedItems,
    #[strum(serialize = "contamination aspects")]
    ContaminationAspects,
    #[strum(serialize = "consider books")]
    ConsiderBooks,
}


/// Read the game's configuration file
/// 
/// returns: Result<HashMap<String, String, RandomState>, Error> 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
fn read_game_config() -> anyhow::Result<HashMap<String, String>> {
    let file_path = get_config_file_path()
        .unwrap_or(bail!("Failed to retrieve configuration file path"));
    let file = std::fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut config = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(config)
}

fn get_autosave_file() -> anyhow::Result<PathBuf> {
    let config = get_game_save_directory()?;
    Ok(config.join("AUTOSAVE.json"))
}

#[tracing::instrument]
fn get_local_low_directory() -> anyhow::Result<PathBuf> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let local_data_dir = dirs::data_dir()
                .inspect(|dir| debug!(?dir, "Retrieved local data directory"))
                .expect("Failed to retrieve local data directory");

            let parent_dir = local_data_dir
                .parent()
                .inspect(|parent| debug!(?parent, "Retrieved AppData parent directory"))
                .expect("Failed to retrieve AppData directory");

            let directory = parent_dir.join("LocalLow");
            info!(?directory, "Constructed LocalLow directory path");

            return Ok(directory);
        }
    }

    bail!("Unsupported platform: Cannot determine local low directory");
}

fn get_game_save_directory() -> anyhow::Result<PathBuf> {
    let mut directory: PathBuf = PathBuf::new();
    #[cfg(target_os = "windows")]
    {
        directory = get_local_low_directory()?
            .join("Weather Factory")
            .join("Book of Hours");
    }

    #[cfg(target_os = "linux")]
    {
        directory = dirs::config_dir()
            .expect("Failed to get config directory")
            .join("unity3d")
            .join("Weather Factory")
            .join("Book of Hours");
    }

    #[cfg(target_os = "macos")]
    {
        directory = dirs::data_local_dir()
            .expect("Failed to get local data directory")
            .join("Weather Factory")
            .join("Book of Hours");
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        bail!("Unsupported platform: Cannot determine game save directory");
    }

    Ok(directory)
}

fn get_config_file_path() -> anyhow::Result<PathBuf> {
    let game_save_directory = get_game_save_directory()?;
    Ok(game_save_directory.join(APP_CONFIG_FILE_NAME))
}

/// Read and parse the configuration using the `confy` crate
/// Loads the `app_config.ini` file under the platform-dependent application data directory
///
/// Returns: Result<PathBuf, Error>
///
/// # Examples
///
/// ```
/// let configured_bhcontent_path = read_config()?;
/// assert_ne!(configured_bhcontent_path.as_os_str().len(), 0);
/// ```
fn read_config() -> anyhow::Result<PathBuf> {
    debug!("Attempting to load application config file using confy");

    // Load the configuration using confy
    let config: Config = confy::load(APP_PATH_FULL, Some("app_config"))?;
    trace!(?config, "Loaded config using confy");

    // Ensure the path exists and is valid
    if config.bhcontent_path.is_empty() {
        eprintln!("Failed to find 'bhcontent_path' value in the config");
        eprintln!("Please configure bhcontent_path to point to a valid dump of the game data");
        eprintln!("Ensure the directory configured contains/end with the path components 'ExportedProject/Assets/StreamingAssets/bhcontent/core'");
        bail!("Configuration is invalid: 'bhcontent_path' is empty");
    }

    Ok(PathBuf::from(config.bhcontent_path))
}

#[tracing::instrument]
fn get_history_file_path(config_path: &PathBuf) -> anyhow::Result<PathBuf> {
    // Ensure path separators match those of the current system
    // This doesn't affect file read/write operations
    // But it makes logged paths cleaner/appear more correct
    // e.g. `evanjs/weatherfactory` (Linux), `evanjs\\weatherfactory` (Windows)
    let config_path_normalized = config_path.canonicalize()?;

    let history_dir = config_path_normalized
        .parent()
        .inspect(|&p| {
            info!(
                directory =? p,
                "Determined application config directory"
            );
        })
        .ok_or_else(|| anyhow::anyhow!("Failed to determine configuration directory."))?;

    // Determine the platform and decide history file name
    let history_file = if cfg!(feature = "with-sqlite-history") {
        history_dir.join("history.sqlite")
    } else if cfg!(feature = "with-file-history") {
        history_dir.join("history.txt")
    } else {
        return Err(anyhow::anyhow!(
            "History handling requires 'with-sqlite-history' or 'with-file-history' feature enabled"
        ));
    };

    info!(?history_file, "Determined history file path");

    Ok(history_file)
}

/// Search the game documents using the provided mode and query
///
/// # Arguments
///
/// * `mode`: the current search mode
/// * `query`: the text query to perform
///
/// returns: Result<Queries, Error>
///
/// # Examples
///
/// ```
/// let tome = get_queries("tomes", "just verse")?;
/// ```
#[tracing::instrument]
fn get_queries(mode: &str, query: &str) -> anyhow::Result<Queries> {
    // Read the base directory from the config
    let base_dir = match read_config() {
        Ok(dir) => dir,
        Err(error) => {
            let message = "Error encountered when attempting to read config file";
            error!(?error, ?mode, ?query, message);
            return Err(anyhow!(message));
        }
    };

    // Construct the file path using the base directory
    let (_bhcontent_path, query_type) = match mode.to_lowercase().as_str() {
        "skills" => (
            format!("{:?}/elements/skills.json", base_dir),
            QueryType::Skills,
        ),
        "aspects" => (
            format!("{:?}/elements/_aspects.json", base_dir),
            QueryType::Aspects,
        ),
        "contamination aspects" => (
            format!("{:?}/elements/contamination_aspects.json", base_dir),
            QueryType::ContaminationAspects,
        ),
        "tomes" => (
            format!("{:?}/elements/tomes.json", base_dir),
            QueryType::Tomes,
        ),
        "aspected items" => (
            format!("{:?}/elements/aspecteditems.json", base_dir),
            QueryType::AspectedItems,
        ),
        "consider books" => (
            format!("{:?}/recipes/1_consider_books.json", base_dir),
            QueryType::ConsiderBooks,
        ),
        _ => return Err(Error::msg(format!("Invalid mode: {}", mode))),
    };

    Ok(Queries {
        name_query: query.to_string(),
        query_type,
    })
}

///
///
/// # Arguments
///
/// * `collection`: a collection containing the objects to be filtered
/// * `query`: the text query to perform
/// * `query_type`: the type of document being queried (TODO: move to `Element`, remove from this function)
/// * `verbose_output`: whether to print extra details about the in-game item
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
///
/// ```
#[tracing::instrument(skip(game_documents, wrapper))]
fn execute_query<W>(
    game_documents: Arc<RwLock<GameDocuments>>,
    wrapper: W,
    query: &str,
    query_type: QueryType,
    verbose_output: bool,
) -> anyhow::Result<()>
where
    W: FindById + GameCollectionType + From<Value>,
    <W as FindById>::Item: Identifiable + GameElementDetails + Debug + Clone + Serialize,
    <W as FindById>::Collection: AsRef<[<W as FindById>::Item]>,
{
    let results = wrapper
        .contains_id_case_insensitive(query)
        .ok_or_else(|| anyhow::anyhow!("Failed to find item using the provided query"))
        .cloned()
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .ok_or(anyhow!("No result found for query: {query}"))?;

    copy_and_print(game_documents, results, query_type, verbose_output)?;
    print_separator();
    Ok(())
}

/// Read the contents of the designated file
///
/// This function is designed to work with both UTF-8 _and_ UTF-16LE files
/// The return type, regardless of the file encoding, will always be UTF-8
/// # Arguments
///
/// * `file_path`: the path to the file to read (can be any type that implements AsRef<Path>)
///
/// returns: Result<String, Error>
///
/// # Examples
///
/// ```
/// let file_contents = read_file_content("elements/_aspects.json")?;
/// assert_ne!(content.chars().count(), 0, "File is empty");
/// ```
fn read_file_content(file_path: &str) -> anyhow::Result<String> {
    let buf = std::fs::read(file_path)?;

    let content = if buf.starts_with(&[0xFF, 0xFE]) {
        // UTF-16 LE with BOM
        let (content, _, had_errors) = UTF_16LE.decode(&buf[2..]);
        if had_errors {
            return Err(anyhow!("Error decoding file: invalid UTF-16 LE"));
        }
        content.into_owned()
    } else {
        // Try decoding as UTF-8
        let (content, _, had_errors) = UTF_8.decode(&buf);
        if had_errors {
            return Err(anyhow!("Error decoding file: invalid UTF-8"));
        }
        content.into_owned()
    };
    Ok(content)
}

fn print_separator() {
    println!("----------------------------------------");
}

/// Print helpful data about a single in-game item (type: `Element`)
/// This function also copies a line to the clipboard for easy input in Excel
///
/// # Copied Data Format
/// The string copied to the clipboard contains a literal <TAB> (`\t`) to ensure
///   that, when copied into Excel, the values are pasted across two columns
///
///     label:\tdescription
///
/// # Arguments
///
/// * `serializable_value`: A single in-game item (type: `Element`)
/// * `query_type`: the type of item being queried (TODO: move to trait on `Element`)
/// * `verbose_output`: whether to print extra details about the in-game item
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// use crate::model::aspects;
/// use crate::model::QueryType;
///
/// let element = aspects::Element::default();
/// copy_and_print(element, QueryType::Aspect
///
/// ```
#[tracing::instrument(skip(game_documents, serializable_value))]
fn copy_and_print<U>(
    game_documents: Arc<RwLock<GameDocuments>>,
    serializable_value: U,
    query_type: QueryType,
    verbose_output: bool,
) -> anyhow::Result<()>
where
    U: Serialize + GameElementDetails + Identifiable + Clone + Debug,
{
    let label = serializable_value.get_label();
    let description = serializable_value.get_desc();

    debug!(?label, ?description, ?query_type, "Found object to print");

    // Copy tab-separated values to clipboard for pasting into Excel
    let combined = format!("{}\t{}", label, description);
    println!("{}", combined);

    copy_if_clipboard_found(combined);

    let has_been_manifested = game_documents
        .read()
        .expect("Failed to get game documents")
        .check_if_item_manifested(&serializable_value)?;
    println!("Already manifested? {}", has_been_manifested);

    let item_from_save_file = match game_documents
        .read()
        .expect("Failed to get game documents")
        .get_item_from_save_file(&serializable_value) {
        Ok(o) => {o}
        Err(e) => {
            let am_i_studying_this = game_documents
                .read()
                .expect("Failed to get game documents")
                .get_studying_item_from_save_file(&serializable_value);

            if let Ok(o) = am_i_studying_this {
                if let true = game_documents
                    .read()
                    .expect("Failed to get game documents")
                    .check_if_tome_mastered(&o) {
                    unreachable!()
                } else {
                    error!(error =? e, "Not printing data for tome being studied!");
                    bail!("Not printing details for tome being studied!");
                }
            } else if let Err(e) = am_i_studying_this {
                bail!("Failed to resolve query for known or studying item!");
            } else {
                unreachable!()
            }
        }
    };

    let have_i_mastered_this = game_documents
        .read()
        .expect("Failed to get game documents")
        .check_if_tome_mastered(&item_from_save_file);


    debug!(
        ?item_from_save_file,
        "Found item from save file"
    );

    if query_type.eq(&QueryType::Tomes) {
        // if querying tomes, check whether found item has been read (i.e. _mastered_)
        if !have_i_mastered_this {
            error!(
                ?label,
                "TOME HAS NOT YET BEEN READ"
            );
            bail!("Not printing details for unread tome!");
        }
    } else {
        // check if item has been crafted

        if !has_been_manifested {
            // Print "label: description"
            println!("{}: {}", label, description);
        } else {
            // Print "label: description"
            error!(
                ?label,
                "Item has not yet been manifested"
            );
            bail!("Not printing details for element not yet manifested!");
        }
    }

    // print each extra item
    if !serializable_value.get_extra().is_empty() {
        for (extra_key, extra_value) in serializable_value
            .get_extra()
            .iter()
            .filter(|(k, v)| k.contains("mastering"))
        {
            let lesson_id = game_documents
                .read()
                .expect("Failed to get game documents")
                .lessons
                .get_lesson_string(extra_value)
                .unwrap_or_else(|| panic!("Failed to get lesson using ID: {extra_key}"));
            println!("{}", lesson_id);
        }
        for (aspected_item_key, aspected_item_value) in serializable_value
            .get_extra()
            .iter()
            .filter(|(k, v)| k.contains("reading"))
        {
            let memory_id = game_documents
                .read()
                .expect("Failed to get game documents")
                .aspected_items
                .get_memory_string(aspected_item_value)
                .unwrap_or_else(|| panic!("Failed to get memory using ID: {aspected_item_key}"));
            println!("{}", memory_id);

            game_documents
                .read()
                .expect("Failed to get game documents")
                .aspected_items
                .get_aspects(aspected_item_value)
                .unwrap_or_else(|| panic!("Failed to get aspect using ID: {aspected_item_key}"))
                .iter()
                .for_each(|(aspect_name, aspect_amount)| {
                    debug!(
                        ?aspected_item_key,
                        ?aspected_item_value,
                        ?aspect_name,
                        ?aspect_amount,
                        "Found aspect to print"
                    );
                    println!("{aspect_name}: {aspect_amount}");
                })
        }
    } else {
        println!("No extra items for {label}");
    }

    // Print full object if `verbose_output` is enabled
    if verbose_output {
        let string = serde_json::to_string_pretty(&serializable_value)?;
        println!("{}", string);
    }

    Ok(())
}

fn copy_if_clipboard_found(text_to_copy: String) {
    match ClipboardContext::new() {
        Ok(clipboard_context) => {
            clipboard_context
                .set_text(text_to_copy.clone())
                .expect("Failed to set clipboard contents");
        }
        Err(error) => {
            debug!(?error, "Failed to get clipboard context");
        }
    }

}

//noinspection ALL,RsUnreachablePatterns
///
///
/// # Arguments
///
/// * `mode`: the current search mode
/// * `query`: the text query to perform
/// * `verbose_output`: whether to print extra details about the in-game item
/// * `query_type`: the type of item being queried (TODO: move to trait on `Element`)
/// * `shared_game_documents`: the shared container of (deserialized) documents from the exported game data
///
/// returns: Result<(), Error>
///
/// # Examples
///
/// ```
/// let documents = Arc::new(GameDocuments::default());
/// process_mode("tomes", "just verse", false, QueryType::Tomes, documents)?;
/// ```
#[tracing::instrument(skip(shared_game_documents))]
fn process_mode(
    mode: &str,
    query: &str,
    verbose_output: bool,
    query_type: QueryType,
    shared_game_documents: Arc<RwLock<GameDocuments>>,
) -> anyhow::Result<()> {
    let queries = match get_queries(mode, query) {
        Ok(q) => q,
        Err(error) => {
            let message = "Error encountered when attempting to get queries";
            error!(
                ?error,
                mode =? mode,
                query =? query,
                ?verbose_output,
                message
            );
            return Err(anyhow!(message));
        }
    };

    let game_documents = shared_game_documents.clone();
    match query_type {
        QueryType::Tomes => {
            trace!(?query_type, "Attempting to get tomes from query");
            let val = game_documents.read().unwrap().tomes.clone();
            execute_query::<Tomes>(
                shared_game_documents.clone(),
                val,
                queries.name_query.as_str(),
                query_type.clone(),
                verbose_output,
            )
        }
        QueryType::Skills => {
            trace!(?query_type, "Attempting to get skills from query");
            let val = game_documents.read().unwrap().skills.clone();
            execute_query::<Skills>(
                shared_game_documents.clone(),
                val,
                queries.name_query.as_str(),
                query_type.clone(),
                verbose_output,
            )
        }
        QueryType::Aspects => {
            trace!(?query_type, "Attempting to get aspects from query");
            let val = game_documents.read().unwrap().aspects.clone();
            execute_query::<Aspects>(
                shared_game_documents.clone(),
                val,
                queries.name_query.as_str(),
                query_type.clone(),
                verbose_output,
            )
        }
        QueryType::ContaminationAspects => {
            trace!(
                ?query_type,
                "Attempting to get contamination aspects from query"
            );
            // let val = game_documents.contamination_aspects.clone();
            // copy_and_print(val, include_object_query)?;
            bail!("Unhandled game document type for game data from json handler")
        }
        QueryType::AspectedItems => {
            trace!(?query_type, "Attempting to get aspected items from query");
            let val = game_documents.read().unwrap().aspected_items.clone();
            execute_query::<AspectedItems>(
                shared_game_documents.clone(),
                val,
                queries.name_query.as_str(),
                query_type.clone(),
                verbose_output,
            )
        }
        QueryType::ConsiderBooks => {
            trace!(?query_type, "Attempting to get consider books from query");
            let val = game_documents.read().unwrap().consider_books.clone();
            execute_query::<ConsiderBooks>(
                shared_game_documents.clone(),
                val,
                queries.name_query.as_str(),
                query_type.clone(),
                verbose_output,
            )
        }
        _ => bail!("Unhandled game document type for game data from json handler"),
    }
}

// Main function with rustyline integration
fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().map_err(|_e| {
        debug!(
            name: "startup",
            "No .env file found. Continuing..."
        );
        warn!(
            name: "startup",
            "TODO: revisit if we need to implement config file loading from e.g. Hub MW configuration files"
        );
    }).ok();

    // Initialize the logger
    let filter = logging::get_env_filter();

    match logging::init_tracing_subscriber(filter) {
        Ok(_) => {
            debug!("Successfully configured logging using provided filter");
        }
        Err(error) => {
            eprintln!(
                "Failed to configure logging using provided filter: {}",
                error
            );
        }
    };

    let bhcontent_core_path = read_config()?;
    let app_config_file_path =
        confy::get_configuration_file_path(APP_PATH_FULL, Some(APP_CONFIG_FILE_NAME))?;
    let game_documents_arc = init_json_data(&bhcontent_core_path)?;

    // Use the receiver in your loop:
    let gda = Arc::clone(&game_documents_arc);
    // Create a crossbeam channel for communicating autosave events
    let (tx, rx): (crossbeam_channel::Sender<notify::Result<Event>>, crossbeam_channel::Receiver<notify::Result<Event>>) = unbounded();

    let mut watcher = recommended_watcher(tx.clone())?;
    let autosave_path = get_autosave_file()?;
    watcher.watch(std::path::Path::new(&autosave_path), RecursiveMode::NonRecursive)?;

    let autosave_flag = Arc::new(AtomicBool::new(true)); // Shared flag to control thread lifecycle
    let autosave_handle = thread::spawn({
        let autosave_flag = Arc::clone(&autosave_flag);
        let autosave_gd = Arc::clone(&game_documents_arc);
        move || {
            loop {
                select! {
                recv(rx) -> msg => {
                    match msg {
                        Ok(result) => match result {
                            Ok(event) => {
                                    match event.kind {
                                        EventKind::Modify(m) => {
                                            match m {
                                                ModifyKind::Data(_data) => {
                                                    debug!(?event, "Autosave file changed");
                                                    trace!("Updating game documents");
                                                    if let Err(error) = update_autosave_document(autosave_gd.clone()) {
                                                        warn!("Error when updating autosave document: {:?}", error);
                                                    } else {
                                                        // TODO: consider promoting the log severity
                                                        //  - when we can more accurately filter events out spammy events
                                                        debug!("Autosave document updated successfully");
                                                    }
                                                }
                                            _ => {}
                                            }
                                        },
                                        _ => {}
                                    }
                            },
                            Err(e) => {
                                println!("autosave watch error: {:?}", e);
                            }
                        },
                        Err(_) => {
                            // Sender (tx) is closed, break loop
                            println!("No more messages. Exiting autosave thread.");
                            break;
                        }
                    }
                }
                default => {
                    // Periodically check the shutdown flag
                    if !autosave_flag.load(Ordering::SeqCst) {
                        println!("Shutdown signal received. Exiting autosave thread.");
                        break;
                    }
                }
            }
            }
        }
    });

    // Create a rustyline Editor instance
    let mut rl = DefaultEditor::new()?;
    let mut mode = String::new();

    #[allow(unused_assignments)]
    let mut include_object_query = false;

    let history_path = get_history_file_path(&app_config_file_path)?;

    maybe_init_history_file(&mut rl, &history_path)?;

    let main_gd = Arc::clone(&game_documents_arc);
    'repl: loop {
        let current_mode_string = format!("{} > ", mode);
        let readline_string = if mode.is_empty() {
            "Enter command ('exit' to quit; 'help' for more commands): ".into()
        } else {
            format!("{}", current_mode_string)
        };

        let readline = rl.readline(readline_string.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let input = line.trim();

                match input {
                    "exit" => break 'repl,
                    "help" => {
                        println!(
                            r##"
Commands:
    reset (return to mode select)
    clear (clear the screen)
Available modes:
    skills
    aspects
    contamination aspects
    tomes
    aspected items
    consider books
                        "##
                        )
                    }
                    "clear" => {
                        rl.clear_screen()?;
                    }
                    "reset" => {
                        mode.clear();
                        println!("Mode reset. Please select a new mode.");
                        continue 'repl;
                    }
                    _ => {
                        if mode.is_empty() {
                            // Set mode if not set
                            mode = input.to_string();
                            if ![
                                "skills",
                                "aspects",
                                "contamination aspects",
                                "tomes",
                                "aspected items",
                                "consider books",
                            ]
                            .contains(&mode.as_str())
                            {
                                error!(input = input, "Invalid mode selected");
                                mode.clear();
                            } else {
                                info!(mode = mode, "Mode set to");
                                println!("Mode set to '{}'. Type your search query or 'reset' to change mode.", mode);
                            }
                        } else {
                            // Process mode function
                            include_object_query = input.ends_with(" -o");
                            let query = if include_object_query {
                                &input[..input.len() - 3]
                            } else {
                                input
                            };
                            trace!(
                                ?mode,
                                ?query,
                                ?include_object_query,
                                "Attempting to process mode"
                            );
                            match process_mode(
                                &mode,
                                query,
                                include_object_query,
                                match mode.parse::<QueryType>() {
                                    Ok(query_type) => query_type,
                                    Err(error) => {
                                        error!(
                                            ?error,
                                            ?mode,
                                            ?query,
                                            ?include_object_query,
                                            "Encountered error when attempting to process mode"
                                        );
                                        continue;
                                    }
                                },
                                Arc::clone(&main_gd),
                            ) {
                                Ok(_) => info!("Command processed: {}", query),
                                Err(error) => {
                                    error!(?error, "Error encountered when running command")
                                }
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break 'repl;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break 'repl;
            }
        }
    }

    drop(tx);
    autosave_flag.store(false, Ordering::SeqCst); // Signal the autosave thread to shut down

    // Wait for the autosave thread to exit
    autosave_handle
        .join()
        .expect("Autosave thread panicked");

    maybe_save_history_file(&mut rl, &history_path)?;

    Ok(())
}

fn update_autosave_document(game_documents: Arc<RwLock<GameDocuments>>) -> anyhow::Result<()> {
    let autosave_path = get_autosave_file()?;
    let autosave_data = load_autosave(autosave_path)?;

    game_documents.write().expect("Failed to get write lock for game documents").autosave = autosave_data;
    Ok(())
}

#[tracing::instrument]
fn load_autosave(autosave_path: PathBuf) -> anyhow::Result<Autosave> {
    let autosave_path_str = autosave_path.to_str().expect("Failed to convert path to string");
    debug!(
        ?autosave_path_str,
        "Attempting to read autosave file"
    );
    let autosave_contents = read_file_content(autosave_path_str)?;
    let autosave: Autosave = serde_json::from_str(&autosave_contents)?;

    Ok(autosave)
}

fn maybe_save_history_file(
    rl: &mut Editor<(), DefaultHistory>,
    history_path: &PathBuf,
) -> anyhow::Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with-sqlite-history")]{
            // Save the command history
            rl.save_history(history_path)?;
        }
        else if #[cfg(feature = "with-file-history")] {
            // Save the command history
            rl.save_history(&history_path)?;
        } else {
            error!("History not saved because neither 'with-sqlite-history' nor 'with-file-history' features are enabled");
        }
    }
    Ok(())
}

fn maybe_init_history_file(
    rl: &mut Editor<(), DefaultHistory>,
    history_path: &PathBuf,
) -> anyhow::Result<()> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with-sqlite-history")] {
             if rl.load_history(&history_path).is_err() {
                info!("No previous history.");
            }
        } else if #[cfg(feature = "with-file-history")] {
            if rl.load_history(&history_path).is_err() {
                info!("No previous history.");
            }
        } else {
            error!("History not loaded because neither 'with-sqlite-history' nor 'with-file-history' features are enabled");
        }
    }

    Ok(())
}

/// Read the contents of the specified JSON file and clean resulting data to ensure
/// the JSON data can be successfully deserialized
///
/// Note: the cleaning operations are done when the data is loaded
/// These operations do _not_ mutate the source files
///
/// Current cleaning operations include:
/// - Convert data from input files with UTF16-LE encoding to UTF-8
/// - Replace instances of CRLF with LF
///
/// # Arguments
///
/// * `file_path`: the path of the JSON file to load
///
/// returns: Result<Value, Error>
///
/// # Examples
///
/// ```
/// let json_file_path: &PathBuf = "recipes\1_consider_books.json".into();
/// let json_file_data = deserialize_json_with_arbitrary_encoding(json_file_path)?;
/// ```
fn deserialize_json_with_arbitrary_encoding(file_path: &PathBuf) -> anyhow::Result<Value> {
    debug!(?file_path, "Attempting to read file");
    let file_contents = read_file_content(&file_path.to_string_lossy())?;
    let json_value: Value = match serde_json_lenient::from_str_lenient(&file_contents) {
        Ok(json_value) => json_value,
        Err(error) => {
            error!(
                ?error,
                ?file_path,
                "Failed to deserialize JSON"
            );
            bail!("Failed to deserialize JSON");
        }
    };
    Ok(json_value)
}

/// Using the provided base directory path, initialize the shared game documents object
///
/// # Arguments
///
/// * `base_directory`: the path to the `core` directory of the exported game data
///
/// returns: Result<Arc<GameDocuments, Global>, Error>
///
/// # Examples
///
/// ```
/// let base_directory_path: &PathBuf = "path_to_core_directory".into();
/// let shared_game_documents = init_json_data(base_directory_path)?;
/// ```
fn init_json_data(base_directory: &PathBuf) -> anyhow::Result<Arc<RwLock<GameDocuments>>> {
    let game = GameDocuments::new_using_data_path(base_directory)?;
    Ok(Arc::new(RwLock::new(game)))
}
