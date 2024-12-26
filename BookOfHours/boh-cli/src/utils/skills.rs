use crate::model::skills::{Element, Skills};
use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde_json::Value;

impl Element {
    pub(crate) fn get_label(&self) -> &str {
        &self.label
    }
}

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.label
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for Skills {
    type Item = Element;
    type Collection = Vec<Element>;

    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self.elements.get_collection_mut()
    }
}

impl From<Value> for Skills {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
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
