use crate::model::consider_books::{ConsiderBooks, Element};
use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde_json::Value;

impl Identifiable for Element {
    // TODO: revisit this function to ensure query interface is intuitive
    //  - Currently, id will match entries such as "study.book.general.hint"
    //  - We might want to use label, e.g. "Mystery and Mastery?"
    fn id(&self) -> &str {
        &self.id
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for ConsiderBooks {
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

impl From<Value> for ConsiderBooks {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl GameCollectionType for ConsiderBooks {
    fn get_collection_type(&self) -> QueryType {
        QueryType::ConsiderBooks
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
