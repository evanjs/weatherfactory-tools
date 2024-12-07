use crate::model::{FindById, Identifiable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aspects {
    pub(crate) elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(rename = "id")]
    pub(crate) element_id: String,
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
    pub(crate) id: String,
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
    fn id(&self) -> &str {
        &self.id
    }
}

impl FindById for Aspects {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}
