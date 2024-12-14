use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Tomes {
    pub(crate) elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    #[serde(rename = "ID")]
    pub(crate) id: String,
    #[serde(rename = "Label")]
    pub(crate) label: String,
    #[serde(rename = "Desc")]
    pub(crate) desc: Option<String>,
    pub(crate) aspects: Option<HashMap<String, i64>>,
    pub(crate) slots: Option<Vec<Slot>>,
    pub(crate) xexts: Option<Xexts>,
    pub(crate) xtriggers: Option<HashMap<String, Vec<Xtrigger>>>,
    pub(crate) inherits: Option<Inherits>,
    pub(crate) unique: Option<bool>,
    pub(crate) audio: Option<Audio>,
    pub(crate) manifestationtype: Option<Audio>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Audio {
    Book,
    Ceramic,
    Scroll,
    #[serde(rename = "SmallWooden")]
    SmallWooden,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Inherits {
    #[serde(rename = "_book")]
    Book,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub(crate) id: Option<Id>,
    pub(crate) label: Option<Label>,
    pub(crate) actionid: Option<Actionid>,
    pub(crate) required: Option<HashMap<String, i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actionid {
    #[serde(rename = "*consider")]
    Consider,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Id {
    Language,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Label {
    Language,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xexts {
    #[serde(rename = "reading.lantern.intro")]
    pub(crate) reading_lantern_intro: Option<String>,
    #[serde(rename = "reading.lantern")]
    pub(crate) reading_lantern: Option<String>,
    #[serde(rename = "reading.grail.intro")]
    pub(crate) reading_grail_intro: Option<String>,
    #[serde(rename = "reading.grail")]
    pub(crate) reading_grail: Option<String>,
    #[serde(rename = "reading.knock.intro")]
    pub(crate) reading_knock_intro: Option<String>,
    #[serde(rename = "reading.knock")]
    pub(crate) reading_knock: Option<String>,
    #[serde(rename = "reading.winter.intro")]
    pub(crate) reading_winter_intro: Option<String>,
    #[serde(rename = "reading.winter")]
    pub(crate) reading_winter: Option<String>,
    #[serde(rename = "reading.nectar.intro")]
    pub(crate) reading_nectar_intro: Option<String>,
    #[serde(rename = "reading.nectar")]
    pub(crate) reading_nectar: Option<String>,
    #[serde(rename = "reading.edge.intro")]
    pub(crate) reading_edge_intro: Option<String>,
    #[serde(rename = "reading.edge")]
    pub(crate) reading_edge: Option<String>,
    #[serde(rename = "reading.moth.intro")]
    pub(crate) reading_moth_intro: Option<String>,
    #[serde(rename = "reading.moth")]
    pub(crate) reading_moth: Option<String>,
    #[serde(rename = "reading.rose.intro")]
    pub(crate) reading_rose_intro: Option<String>,
    #[serde(rename = "reading.rose")]
    pub(crate) reading_rose: Option<String>,
    #[serde(rename = "reading.sky.intro")]
    pub(crate) reading_sky_intro: Option<String>,
    #[serde(rename = "reading.sky")]
    pub(crate) reading_sky: Option<String>,
    #[serde(rename = "reading.forge.intro")]
    pub(crate) reading_forge_intro: Option<String>,
    #[serde(rename = "reading.forge")]
    pub(crate) reading_forge: Option<String>,
    #[serde(rename = "reading.moon.intro")]
    pub(crate) reading_moon_intro: Option<String>,
    #[serde(rename = "reading.moon")]
    pub(crate) reading_moon: Option<String>,
    #[serde(rename = "reading.heart.intro")]
    pub(crate) reading_heart_intro: Option<String>,
    #[serde(rename = "reading.heart")]
    pub(crate) reading_heart: Option<String>,
    #[serde(rename = "reading.scale.intro")]
    pub(crate) reading_scale_intro: Option<String>,
    #[serde(rename = "reading.scale")]
    pub(crate) reading_scale: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Xtrigger {
    pub(crate) id: Option<String>,
    pub(crate) morpheffect: Option<Morpheffect>,
    pub(crate) level: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Morpheffect {
    Spawn,
}

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.label
    }
}

// Implement the trait for some struct
impl<T: Identifiable> FindById for Vec<T> {
    type Item = T;
    type Collection = Vec<T>;

    fn get_collection(&self) -> &Self::Collection {
        self
    }
}

impl FindById for Tomes {
    type Item = Element;

    type Collection = Vec<Element>;

    #[tracing::instrument(skip(self))]
    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}

impl From<Value> for Tomes {
    fn from(value: Value) -> Self {
        serde_json_path_to_error::from_value(value).unwrap()
    }
}

impl GameCollectionType for Tomes {
    fn get_collection_type(&self) -> QueryType {
        QueryType::Tomes
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

    #[tracing::instrument(skip(self))]
    fn get_extra(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        match &self.xtriggers {
            None => {
                warn!("No xtriggers found for object");
                map
            }
            Some(s) => {
                let m = s.clone();

                let (mk, mv) = m
                    .clone()
                    .iter()
                    .filter(|(k, v)| {
                        info!(
                            key =? k,
                            value =? v,
                            "Checking if mastering ID starts with query"
                        );
                        k.starts_with("mastering")
                    })
                    .map(|(k, v)| (k.clone(), v.first().unwrap().id.clone().unwrap()))
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .clone();

                map.insert(mk, mv);

                let (rk, rv) = m
                    .clone()
                    .into_iter()
                    .filter(|(k, v)| {
                        debug!(
                            key =? k,
                            value =? v,
                            "Checking if reading ID starts with query"
                        );
                        k.starts_with("reading")
                    })
                    .map(|(k, v)| (k.clone(), v.first().unwrap().id.clone().unwrap()))
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .clone();

                map.insert(rk, rv);

                info!(?map, "Produced extra map");
                map
            }
        }
    }
}
