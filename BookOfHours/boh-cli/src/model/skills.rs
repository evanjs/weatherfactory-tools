use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Skills {
    pub(crate) elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub(crate) id: String,
    #[serde(rename = "Label")]
    pub(crate) label: String,
    #[serde(rename = "Desc")]
    pub(crate) desc: Option<String>,
    pub(crate) aspects: Option<HashMap<String, i64>>,
    pub(crate) ambits: Option<HashMap<String, i64>>,
    pub(crate) xtriggers: Option<Xtriggers>,
    pub(crate) slots: Option<Vec<Slot>>,
    #[serde(rename = "AlphaLabelOverride")]
    pub(crate) alpha_label_override: Option<String>,
}

impl Element {
    pub(crate) fn get_label(&self) -> &str {
        &self.label
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub(crate) id: Option<SlotId>,
    pub(crate) label: Option<Label>,
    pub(crate) actionid: Option<Actionid>,
    pub(crate) essential: Option<Essential>,
    pub(crate) required: Option<HashMap<String, i64>>,
    pub(crate) ifaspectspresent: Option<Ifaspectspresent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Actionid {
    Consider,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Essential {
    pub(crate) ability: Option<i64>,
    pub(crate) lesson: Option<i64>,
    pub(crate) memory: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotId {
    A1,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ifaspectspresent {
    pub(crate) skill: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Label {
    Effort,
    Lesson,
    Memory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xtriggers {
    pub(crate) skillingup: Option<Vec<Skillingup>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skillingup {
    pub(crate) id: Option<SkillingupId>,
    pub(crate) morpheffect: Option<Morpheffect>,
    pub(crate) level: Option<i64>,
    pub(crate) additive: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillingupId {
    Edge,
    Forge,
    Grail,
    Heart,
    Knock,
    Lantern,
    Moon,
    Moth,
    Nectar,
    Rose,
    Scale,
    Skill,
    Sky,
    Winter,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Morpheffect {
    Mutate,
}

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.label
    }
    fn inner_id(&self) -> &str { &self.id }
}

impl FindById for Skills {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}

impl From<Value> for Skills {
    fn from(value: Value) -> Self {
        serde_json_path_to_error::from_value(value).unwrap()
    }
}

impl GameCollectionType for Skills {
    fn get_collection_type(&self) -> QueryType {
        QueryType::Skills
    }
}

impl GameElementDetails for Element {
    fn get_label(&self) -> String {
        self.clone().label
    }
    fn get_desc(&self) -> String {
        let a = self.clone().desc;
        let b = a.unwrap_or_default();
        b.clone()
    }
}
