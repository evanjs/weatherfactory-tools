use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
pub(crate) struct Config {
    pub(crate) bhcontent_path: String,
}

