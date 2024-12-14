use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) bhcontent_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bhcontent_path: String::new()
        }
    }
}