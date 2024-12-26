use crate::model::aspected_items::{AspectedItems, Element};
use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde_json::Value;
use std::collections::HashMap;
use tracing::trace;

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.id
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for AspectedItems {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }

    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self.elements.get_collection_mut()
    }
}

impl From<Value> for AspectedItems {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
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

    #[tracing::instrument(skip(self))]
    fn get_extra(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        if let Some(aspects) = &self.aspects {
            let mapped_aspects = aspects
                .iter()
                .map(|(k, v)| (k.clone(), v.clone().to_string()))
                .collect::<HashMap<String, String>>();

            map.extend(mapped_aspects)
        }

        map
    }
}

impl AspectedItems {
    #[tracing::instrument(skip(self))]
    pub(crate) fn get_memory_from_id(&self, id: &str) -> Option<&Element> {
        self.elements.iter().find(|&f| {
            trace!(
                existing_id =? &f.id,
                queried_id =? id,
                "Checking if query matches ID"
            );
            f.id.as_str() == id
        })
    }
    #[tracing::instrument(skip(self))]
    pub(crate) fn get_memory_string_from_id(&self, id: &str) -> Option<String> {
        self.elements
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
    }

    #[tracing::instrument(skip(self))]
    pub(crate) fn get_memory_string_from_query(&self, query: &str) -> Option<String> {
        self.elements
            .iter()
            .find(|&f| {
                trace!(
                    memory_label =? &f.get_label(),
                    ?query,
                    "Checking if memory label contains query"
                );
                f.get_label().eq_ignore_ascii_case(query)
            })
            .map(|e| e.label.clone())
    }

    pub(crate) fn get_aspects(&self, id: &str) -> Option<HashMap<String, String>> {
        self.elements
            .iter()
            .find(|&f| f.id().eq_ignore_ascii_case(id))
            .and_then(|element| {
                element.aspects.as_ref().map(|aspects| {
                    aspects
                        .iter()
                        .filter(|(key, _)| !key.contains("boost"))
                        .map(|(key, value)| (key.clone(), value.to_string()))
                        .collect()
                })
            })
    }
}
