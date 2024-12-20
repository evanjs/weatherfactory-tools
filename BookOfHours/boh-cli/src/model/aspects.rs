use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Aspects {
    pub(crate) elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "id")]
    pub(crate) element_id: Option<String>,
    #[serde(rename = "label")]
    pub(crate) element_label: Option<String>,
    pub(crate) ishidden: Option<bool>,
    pub(crate) noartneeded: Option<bool>,
    pub(crate) desc: Option<String>,
    pub(crate) is_aspect: Option<bool>,
    pub(crate) sort: Option<String>,
    pub(crate) is_hidden: Option<IsHidden>,
    pub(crate) icon: Option<String>,
    pub(crate) comments: Option<String>,
    pub(crate) isaspect: Option<bool>,
    pub(crate) inherits: Option<Inherits>,
    pub(crate) ambits: Option<HashMap<String, i64>>,
    pub(crate) commute: Option<Vec<String>>,
    #[serde(rename = "ID")]
    pub(crate) id: Option<String>,
    #[serde(rename = "Label")]
    pub(crate) label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Inherits {
    #[serde(rename = "_inspiration")]
    Inspiration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IsHidden {
    Bool(bool),
    String(String),
}

impl Identifiable for Element {
    /// TODO: review [Identifiable::id()] for each implementor to determine what makes sense
    ///   - One of the ID variants might not be available in all cases
    fn id(&self) -> &str {
        match (&self.id, &self.element_id) {
            (a, None) => a.as_ref().unwrap(),
            (None, b) => b.as_ref().unwrap(),
            // Is this valid?
            // Will/do any items have both id _and_ ID?
            (a, b) => b.as_ref().unwrap(),
        }
    }

    fn inner_id(&self) -> &str {
        match (&self.id, &self.element_id) {
            (a, None) => a.as_ref().unwrap(),
            (None, b) => b.as_ref().unwrap(),
            // Is this valid?
            // Will/do any items have both id _and_ ID?
            (a, b) => b.as_ref().unwrap(),
        }
    }
}

impl FindById for Aspects {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}

impl From<Value> for Aspects {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).expect("Failed to parse Aspects value")
    }
}

impl GameCollectionType for Aspects {
    fn get_collection_type(&self) -> QueryType {
        QueryType::Aspects
    }
}

impl GameElementDetails for Element {
    fn get_label(&self) -> String {
        let a = self.clone().label;
        let b = a.unwrap_or_default();
        b.clone()
    }
    fn get_desc(&self) -> String {
        let a = self.clone().desc;
        let b = a.unwrap_or_default();
        b.clone()
    }
}
