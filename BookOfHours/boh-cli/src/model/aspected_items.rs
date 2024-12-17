use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AspectedItems {
    pub(crate) elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    #[serde(rename = "ID")]
    pub(crate) id: String,
    pub(crate) uniquenessgroup: Option<String>,
    #[serde(rename = "Label")]
    pub(crate) label: String,
    #[serde(rename = "Desc")]
    pub(crate) desc: Option<String>,
    pub(crate) inherits: Option<String>,
    pub(crate) audio: Option<String>,
    pub(crate) aspects: Option<HashMap<String, i64>>,
    pub(crate) xtriggers: Option<Xtriggers>,
    pub(crate) xexts: Option<Xexts>,
    pub(crate) unique: Option<bool>,
    pub(crate) ambits: Option<Ambits>,
    pub(crate) icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ambits {
    #[serde(rename = "sack.vegetables")]
    pub(crate) sack_vegetables: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xexts {
    pub(crate) scrutiny: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xtriggers {
    pub(crate) recovering: Option<String>,
    pub(crate) scrutiny: Option<Vec<Dist>>,
    pub(crate) dist: Option<Vec<Dist>>,
    pub(crate) fatiguing: Option<String>,
    #[serde(rename = "fatiguing.ingredients")]
    pub(crate) fatiguing_ingredients: Option<String>,
    pub(crate) cleaning: Option<String>,
    pub(crate) dissipating: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dist {
    pub(crate) id: Option<String>,
    pub(crate) morpheffect: Option<Morpheffect>,
    pub(crate) level: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Morpheffect {
    Spawn,
    Transform,
}

impl Identifiable for Element {
    fn id(&self) -> &str { &self.id }
    fn inner_id(&self) -> &str { &self.id }
}

impl FindById for AspectedItems {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}

impl From<Value> for AspectedItems {
    fn from(value: Value) -> Self {
        serde_json_path_to_error::from_value(value).unwrap()
    }
}

impl GameCollectionType for AspectedItems {
    fn get_collection_type(&self) -> QueryType {
        QueryType::AspectedItems
    }
}

impl GameElementDetails for Element {
    fn get_label(&self) -> String {
        self.label.clone()
    }
    fn get_desc(&self) -> String {
        let a = self.clone().desc;
        let b = a.unwrap_or_default();
        b.clone()
    }
}

impl AspectedItems {
    #[tracing::instrument(skip(self))]
    pub(crate) fn get_memory_string(&self, id: &str) -> Option<String> {
        self.elements
            .iter()
            .find(|&f| {
                info!(
                    existing_id =? &f.id,
                    queried_id =? id,
                    "Checking if query matches ID"
                );
                f.id.as_str() == id
            })
            .map(|e| e.label.clone())
    }

    pub(crate) fn get_aspects(&self, id: &str) -> Option<HashMap<String, String>> {
        self.elements
            .iter()
            .find(|&f| f.id.as_str() == id)
            .and_then(|element| {
                element.aspects.as_ref().map(|aspects| {
                    aspects
                        .iter()
                        .filter(|(key, value)| !key.contains("boost"))
                        .map(|(key, value)| (key.clone(), value.to_string()))
                        .collect()
                })
            })
    }
}
