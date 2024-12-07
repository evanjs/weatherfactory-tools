mod logging;
mod model;

use std::fs;
use std::fs::File;
use tracing::{debug, error, info, trace, warn};

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::Read;

use crate::model::aspects::Aspects;
use crate::model::skills::Skills;
use crate::model::tomes::Tomes;
use crate::model::{skills, tomes, FindById};
use anyhow::{anyhow, bail, Error};
use clipboard_rs::{Clipboard, ClipboardContext};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
struct Queries {
    bhcontent_path: String,
    name_query: String,
    query_type: QueryType,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
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
}

// Function to read configuration
#[tracing::instrument]
fn read_config() -> anyhow::Result<String> {
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Value = serde_json::from_str(&contents)?;
    if let Some(base_path) = config["bhcontent_path"].as_str() {
        Ok(base_path.to_string())
    } else {
        Err(anyhow::Error::msg(
            "Missing 'bhcontent_path' in config.json",
        ))
    }
}

// Function to get queries based on mode and query
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
    let (bhcontent_path, query_type) = match mode.to_lowercase().as_str() {
        "skills" => (
            format!("{}/elements/skills.json", base_dir),
            QueryType::Skills,
        ),
        "aspects" => (
            format!("{}/elements/_aspects.json", base_dir),
            QueryType::Aspects,
        ),
        "contamination aspects" => (
            format!("{}/elements/contamination_aspects.json", base_dir),
            QueryType::ContaminationAspects,
        ),
        "tomes" => (
            format!("{}/elements/tomes.json", base_dir),
            QueryType::Tomes,
        ),
        "aspected items" => (
            format!("{}/elements/aspecteditems.json", base_dir),
            QueryType::AspectedItems,
        ),
        _ => return Err(Error::msg(format!("Invalid mode: {}", mode))),
    };

    Ok(Queries {
        bhcontent_path: bhcontent_path.to_string(),
        name_query: query.to_string(),
        query_type,
    })
}

fn determine_value_type(json_val: &Value) -> anyhow::Result<(String, String)> {
    if let Ok(tome) = serde_json::from_value::<tomes::Element>(json_val.clone()) {
        Ok((tome.label, tome.desc.unwrap_or("N/A".to_string())))
    } else if let Ok(skill) = serde_json::from_value::<skills::Element>(json_val.clone()) {
        Ok((skill.label, skill.desc.unwrap_or("N/A".to_string())))
    } else {
        bail!("Failed to determine the type of JSON value");
    }
}

// Dummy implementations for querying that would need to be replaced with actual logic.
#[tracing::instrument(skip(json))]
fn execute_query<'a>(json: &Value, query: &str, query_type: &QueryType) -> anyhow::Result<Value> {
    // Implement JSON querying logic here
    match query_type {
        QueryType::Tomes => {
            trace!(?query, ?query_type, "Attempting to get tome from query");
            match serde_json::from_value::<Tomes>(json.clone()) {
                Ok(o) => {
                    let tome: Option<&tomes::Element> = o.contains_id_case_insensitive(query);
                    let t = match tome {
                        Some(t) => {
                            let o = Ok(serde_json::to_value(t).unwrap_or(serde_json::Value::Null))
                                .inspect(|r| {
                                    let j = serde_json::to_string_pretty(&r)
                                        .unwrap_or("null".to_string());
                                    info!(
                                        %j,
                                        "Got tomes"
                                    );
                                });
                            o
                        }
                        None => {
                            let message = "Failed to deserialize JSON";
                            error!(?query, ?query_type, message);
                            bail!(message)
                        }
                    };
                    //dbg!(&t);
                    t
                }
                Err(error) => {
                    let message = "Failed to deserialize JSON";
                    error!(?error, message);
                    bail!(message)
                }
            }
        }
        _ => {
            let message = "Query type not handled";
            error!(?query, ?query_type, message);
            bail!(message)
        }
    }
}

// Function to fetch and display the results
#[tracing::instrument]
fn fetch_and_display(
    queries: &Queries,
    include_object_query: bool,
    query_type: QueryType,
) -> anyhow::Result<()> {
    debug!(
        queries_bhcontent_path =? queries.bhcontent_path,
        "Attempting to read file"
    );

    let file_content = match fs::read_to_string(&queries.bhcontent_path) {
        Ok(o) => o,
        Err(error) => {
            let message = "Error encountered when attempting to read file";
            error!(
                ?error,
                queries_bhcontent_path =? queries.bhcontent_path,
                message
            );
            return Err(anyhow!(message));
        }
    };
    let json_value: Value = serde_json::from_str(&file_content)?;

    let a = execute_query(&json_value, &queries.name_query, &query_type)?;
    copy_and_print(a, include_object_query)?;
    Ok(())
}

#[tracing::instrument(skip(serializable_value))]
fn copy_and_print(serializable_value: Value, print_object: bool) -> anyhow::Result<()> {
    if let Ok((key, value)) = determine_value_type(&serializable_value) {
        let ctx = ClipboardContext::new().expect("Failed to get clipboard context");

        // Copy tab-separated values to clipboard for pasting into Excel
        let combined = format!("{}\t{}", key, value);
        ctx.set_text(combined.clone())
            .expect("Failed to set clipboard contents");
        // Print
        if print_object {
            println!("{:#?}", combined);
        }
    } else {
        bail!("Failed to determine the type of JSON value");
    }

    Ok(())
}

#[tracing::instrument]
fn process_mode(
    mode: &str,
    query: &str,
    include_object_query: bool,
    query_type: QueryType,
) -> anyhow::Result<()> {
    let queries = match get_queries(mode, query) {
        Ok(q) => q,
        Err(error) => {
            let message = "Error encountered when attempting to get queries";
            error!(
                ?error,
                mode =? mode,
                query =? query,
                ?include_object_query,
                message
            );
            return Err(anyhow!(message));
        }
    };
    fetch_and_display(&queries, include_object_query, query_type)
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
            println!("Successfully configured logging using provided filter");
        }
        Err(error) => {
            eprintln!(
                "Failed to configure logging using provided filter: {}",
                error
            );
        }
    };

    // Create a rustyline Editor instance
    let mut rl = DefaultEditor::new()?;
    let mut mode = String::new();

    #[allow(unused_assignments)]
    let mut include_object_query = false;

    cfg_if::cfg_if! {
        if #[cfg(feature = "with-sqlite-history")] {
             if rl.load_history("history.sqlite").is_err() {
                info!("No previous history.");
            }
        } else if #[cfg(feature = "with-file-history")] {
            if rl.load_history("history.txt").is_err() {
                info!("No previous history.");
            }
        } else {
            error!("History not loaded because neither 'with-sqlite-history' nor 'with-file-history' features are enabled");
        }
    }

    'repl: loop {
        let readline = rl.readline("Enter command (or 'exit' to quit): ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let input = line.trim();

                match input {
                    "exit" => break 'repl,
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
                            ) {
                                Ok(_) => println!("Command processed: {}", query),
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

    cfg_if::cfg_if! {
        if #[cfg(feature = "with-sqlite-history")]{
            // Save the command history
            rl.save_history("history.sqlite")?;
        }
        else if #[cfg(feature = "with-file-history")] {
            // Save the command history
            rl.save_history("history.txt")?;
        } else {
            error!("History not saved because neither 'with-sqlite-history' nor 'with-file-history' features are enabled");
        }
    }

    Ok(())
}
