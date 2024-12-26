use std::collections::HashMap;
use tracing::{debug, trace, warn};
use serde_json::Value;
use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::model::tomes::{Element, Tomes};
use crate::QueryType;

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.label
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for Tomes {
    type Item = Element;

    type Collection = Vec<Element>;

    #[tracing::instrument(skip(self))]
    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }

    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self.elements.get_collection_mut()
    }
}

impl From<Value> for Tomes {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
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
                        trace!(
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

                trace!(?map, "Produced extra map");
                map
            }
        }
    }
}