use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Config {
    pub(crate) bhcontent_path: String,
}
