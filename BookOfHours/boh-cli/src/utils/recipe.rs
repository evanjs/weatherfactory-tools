use serde_json::Value;
use crate::model::{BoolOrString, FindById, GameElementDetails, Identifiable};
use crate::model::recipe::{Element, Recipes};

impl IntoIterator for Recipes {
    type Item = Element;
    type IntoIter = std::vec::IntoIter<Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl From<BoolOrString> for bool {
    fn from(val: BoolOrString) -> Self {
        match val {
            BoolOrString::Bool(b) => b,
            BoolOrString::String(s) => s.parse().unwrap_or(false),
        }
    }
}

impl From<Value> for Recipes {
    fn from(value: Value) -> Self {
        serde_json_path_to_error::from_value(value).unwrap()
    }
}

impl From<Recipes> for Value {
    fn from(val: Recipes) -> Self {
        serde_json::to_value(val).unwrap()
    }
}

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.id
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for Recipes {
    type Item = Element;

    type Collection = Vec<Element>;

    #[tracing::instrument(skip(self))]
    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }

    #[tracing::instrument(skip(self))]
    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self.elements.get_collection_mut()
    }
}

impl GameElementDetails for Element {
    fn get_label(&self) -> String {
        self.label.as_ref().unwrap_or(&"N/A".to_string()).clone()
    }
    fn get_desc(&self) -> String {
        self.clone().desc.unwrap_or_default().clone()
    }
}