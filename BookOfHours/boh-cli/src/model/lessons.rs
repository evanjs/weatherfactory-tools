use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lessons {
    pub(crate) elements: Option<Vec<Element>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub(crate) id: String,
    #[serde(rename = "Label")]
    pub(crate) label: String,
    pub(crate) desc: String,
    pub(crate) aspects: Option<HashMap<String, i64>>,
    pub(crate) inherits: Option<Inherits>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Inherits {
    #[serde(rename = "_memory.persistent")]
    MemoryPersistent,
}
