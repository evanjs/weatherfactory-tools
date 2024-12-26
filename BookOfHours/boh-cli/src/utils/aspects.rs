use crate::model::aspects::{Aspects, Element};
use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde_json::Value;

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
    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self.elements.get_collection_mut()
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
