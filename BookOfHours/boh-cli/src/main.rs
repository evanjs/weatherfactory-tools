mod logging;

use std::fs;
use std::fs::File;
use tracing::{debug, error, info, warn};

use std::io::{self, Read, Write};

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use anyhow::{anyhow, Error};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tracing_subscriber::util::TryInitError;

#[derive(Debug, Serialize, Deserialize)]
struct Queries {
    bhcontent_path: String,
    name_query: String,
    value_query: String,
    object_query: String,
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
        Err(anyhow::Error::msg("Missing 'bhcontent_path' in config.json"))
    }
}

// Function to get queries based on mode and query
fn get_queries(mode: &str, query: &str) -> anyhow::Result<Queries> {

    // Read the base directory from the config
    let base_dir = match read_config() {
        Ok(dir) => dir,
        Err(error) => {
            let message = "Error encountered when attempting to read config file";
            error!(
                ?error,
                ?mode,
                ?query,
                message
            );
            return Err(anyhow!(message))
        }
    };

    let base_selector = r#"((.Label // .label) // .id) | select(. != null) | test("#;
    let name_selector = r#"(.Label // .label // .id)"#;

    let name_query = format!(
        ".[][] | select({}{}) | {} | select(.)",
        base_selector, query, name_selector
    );
    let value_query = format!(
        ".[][] | select({}{}) | (.desc // .Desc) | select(.)",
        base_selector, query
    );
    let object_query = format!(".[][] | select({}{})", base_selector, query);

    // Construct the file path using the base directory
    let bhcontent_path = match mode.to_lowercase().as_str() {
        "skills" => format!("{}/elements/skills.json", base_dir),
        "aspects" => format!("{}/elements/_aspects.json", base_dir),
        "contamination aspects" => format!("{}/elements/contamination_aspects.json", base_dir),
        "tomes" => format!("{}/elements/tomes.json", base_dir),
        "aspected items" => format!("{}/elements/aspecteditems.json", base_dir),
        _ => return Err(Error::msg(format!("Invalid mode: {}", mode))),
    };

    Ok(Queries {
        bhcontent_path: bhcontent_path.to_string(),
        name_query,
        value_query,
        object_query,
    })
}

// Function to fetch and display the results
#[tracing::instrument]
fn fetch_and_display(queries: &Queries, include_object_query: bool) -> anyhow::Result<()> {
    debug!(
        queries_bhcontent_path =? queries.bhcontent_path,
        "Attempting to read file"
    );

    let file_content = match fs::read_to_string(&queries.bhcontent_path) {
        Ok(o) => {
            o
        }
        Err(error) => {
            let message = "Error encountered when attempting to read file";
            error!(
                ?error,
                queries_bhcontent_path =? queries.bhcontent_path,
                message
            );
            return Err(anyhow!(message))
        }
    };
    let json_value: Value = serde_json::from_str(&file_content)?;

    // Dummy implementations for querying that would need to be replaced with actual logic.
    fn execute_query(json: &Value, query: &str) -> Option<String> {
        // Implement JSON querying logic here
        Some(query.to_string()) // Placeholder
    }

    let name = execute_query(&json_value, &queries.name_query).unwrap_or_default();
    let value = execute_query(&json_value, &queries.value_query).unwrap_or_default();

    if include_object_query {
        let object = execute_query(&json_value, &queries.object_query).unwrap_or_default();
        println!("Full Object:\n{}", object);
    }

    println!("Name: {}\nDescription: {}", name, value);
    Ok(())
}

fn process_mode(mode: &str, query: &str, include_object_query: bool) -> anyhow::Result<()> {
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
            return Err(anyhow!(message))
        }
    };
    fetch_and_display(&queries, include_object_query)
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
            eprintln!("Failed to configure logging using provided filter: {}", error);
        }
    };

    // Create a rustyline Editor instance
    let mut rl = DefaultEditor::new()?;
    let mut mode = String::new();
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

    loop {
        let readline = rl.readline("Enter command (or 'exit' to quit): ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let input = line.trim();

                match input {
                    "exit" => break,
                    "reset" => {
                        mode.clear();
                        println!("Mode reset. Please select a new mode.");
                        continue;
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
                            match process_mode(&mode, query, include_object_query) {
                                Ok(_) => println!("Command processed: {}", query),
                                Err(error) => error!(
                                    ?error,
                                    "Error encountered when running command"
                                ),
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
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

fn process_command(input: &str) -> anyhow::Result<String> {
    // Dummy implementation
    // Parse the command and execute corresponding function
    Ok(format!("Command processed: {}", input))
}
