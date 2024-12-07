use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::model::{FindById, Identifiable};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    fn id(&self) -> &str {
        &self.id
    }
}

impl FindById for AspectedItems {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}