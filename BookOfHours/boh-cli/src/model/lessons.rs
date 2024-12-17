use crate::model::Identifiable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{info, trace, warn};

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

impl Lessons {
    #[tracing::instrument(skip(self))]
    pub(crate) fn get_lesson_string(&self, id: &str) -> Option<String> {
        if let Some(elements) = &self.elements {
            Some(
                elements
                    .iter()
                    .find(|&f| {
                        trace!(
                            existing_id =? &f.id,
                            queried_id =? id,
                            "Checking if query matches ID"
                        );
                        f.id.as_str() == id
                    })
                    .map(|e| e.label.clone())
                    .unwrap_or_else(|| format!("No lesson found for id {}", id)),
            )
        } else {
            warn!("No lessons found for ID: {id}");
            None
        }
    }
}

impl From<Value> for Lessons {
    fn from(value: Value) -> Self {
        serde_json_path_to_error::from_value(value).unwrap()
    }
}
