mod log;

use std::fs;
use tracing::{error, info};

use std::io::{self, Write};

use anyhow::Error;
use serde_json::Value;

struct Queries {
    file_path: String,
    name_query: String,
    value_query: String,
    object_query: String,
}

// Function to get queries based on mode and query
fn get_queries(mode: &str, query: &str) -> anyhow::Result<Queries> {
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

    let file_path = match mode.to_lowercase().as_str() {
        "skills" => "./elements/skills.json",
        "aspects" => "./elements/_aspects.json",
        "contamination aspects" => "./elements/contamination_aspects.json",
        "tomes" => "./elements/tomes.json",
        "aspected items" => "./elements/aspecteditems.json",
        _ => return Err(Error::msg(format!("Invalid mode: {}", mode))),
    };

    Ok(Queries {
        file_path: file_path.to_string(),
        name_query,
        value_query,
        object_query,
    })
}

// Function to fetch and display the results
fn fetch_and_display(queries: &Queries, include_object_query: bool) -> anyhow::Result<()> {
    let file_content = fs::read_to_string(&queries.file_path)?;
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
    let queries = get_queries(mode, query)?;
    fetch_and_display(&queries, include_object_query)
}

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

// Main function with rustyline integration
fn main() -> Result<(), ReadlineError> {
    // Initialize the logger
    log::init_logging();

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
                                Err(err) => eprintln!("Error: {}", err),
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
