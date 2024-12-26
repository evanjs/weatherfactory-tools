use crate::QueryType;
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use tracing::{debug, trace, warn};

pub(crate) mod aspected_items;
pub(crate) mod aspects;
pub(crate) mod config;
pub(crate) mod consider_books;
pub(crate) mod game_documents;
pub(crate) mod lessons;
pub(crate) mod recipe;
pub(crate) mod save;
pub(crate) mod skills;
pub(crate) mod tomes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoolOrString {
    Bool(bool),
    String(String),
}

// Define a shared trait for elements that have an ID field
pub trait Identifiable {
    fn id(&self) -> &str;
    fn inner_id(&self) -> &str;
}

pub trait Mastery {
    fn has_mastery(&self) -> bool;
}

pub trait GameCollectionType {
    fn get_collection_type(&self) -> QueryType;
}

pub trait GameElementDetails {
    fn get_label(&self) -> String;
    fn get_desc(&self) -> String;
    fn get_extra(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

// Define a trait for collections that can find elements by ID
pub trait FindById {
    type Item: Identifiable + GameElementDetails;

    type Collection: IntoIterator<Item = Self::Item>;

    fn get_collection(&self) -> &Self::Collection;
    fn get_collection_mut(&mut self) -> &mut Self::Collection;

    #[tracing::instrument(skip(self))]
    fn find_by_id(&self, id: &str) -> Option<&Self::Item>
    where
        Self::Collection: AsRef<[Self::Item]>,
    {
        self.get_collection()
            .as_ref()
            .iter()
            .find(|&element| element.id() == id)
    }

    #[tracing::instrument(skip(self))]
    fn find_by_id_case_insensitive(&self, id: &str) -> Option<&Self::Item>
    where
        Self::Collection: AsRef<[Self::Item]>,
    {
        self.get_collection()
            .as_ref()
            .iter()
            .find(|&element| element.id().eq_ignore_ascii_case(id))
    }

    #[tracing::instrument(skip(self))]
    fn label_contains_query_case_insensitive(&self, query: &str) -> Option<&Self::Item>
    where
        Self::Collection: AsRef<[Self::Item]>,
        <Self as FindById>::Item: Debug,
    {
        debug!(
            ?query,
            "Searching for element with provided label (case insensitive)"
        );

        self.get_collection().as_ref().iter().find(|&element| {
            trace!(
                id =? element.id(),
                label = element.get_label(),
                ?query,
                "Checking if label contains query (case insensitive)"
            );
            element
                .get_label()
                .to_ascii_lowercase()
                .contains(&query.to_ascii_lowercase())
        })
    }

    #[tracing::instrument(skip(self))]
    fn contains_id_case_insensitive(&self, id: &str) -> Option<&Self::Item>
    where
        Self::Collection: AsRef<[Self::Item]>,
        <Self as FindById>::Item: Debug,
    {
        debug!(
            query_id =? id,
            "Searching for element with provided ID (case insensitive)"
        );

        self.get_collection()
            .as_ref()
            .iter()
            .find(|element| {
                trace!(
                    element_id =? element.id(),
                    query_id =? id,
                    "Checking if element id contains id (case insensitive)"
                );
                element
                    .id()
                    .to_ascii_lowercase()
                    .contains(&id.to_ascii_lowercase())
            })
            .inspect(|element| {
                trace!(
                    element =? debug(element),
                    element_id =? element.id(),
                    query_id =? id,
                    "Found element with provided ID"
                );
            })
            .or_else(|| {
                warn!(
                    element_id =? id,
                    query_id =? id,
                    "No element with provided ID found"
                );
                None
            })
    }

    #[tracing::instrument(skip(self))]
    fn contains_id(&self, id: &str) -> Option<&Self::Item>
    where
        Self::Collection: AsRef<[Self::Item]>,
        <Self as FindById>::Item: Debug,
    {
        debug!(
            query_id =? id,
            "Searching for element with provided ID"
        );
        self.get_collection()
            .as_ref()
            .iter()
            .find(|element| {
                trace!(
                    element_id =? element.id(),
                    query_id =? id,
                    "Checking if element id contains id"
                );
                element.id().contains(id)
            })
            .inspect(|element| {
                trace!(
                    element =? debug(element),
                    element_id =? element.id(),
                    query_id =? id,
                    "Found element with provided ID"
                );
            })
            .or_else(|| {
                warn!(
                    element_id =? id,
                    query_id =? id,
                    "No element with provided ID found"
                );
                None
            })
    }
}

// Implement the trait for some struct
impl<T: Identifiable + GameElementDetails> FindById for Vec<T> {
    type Item = T;
    type Collection = Vec<T>;

    fn get_collection(&self) -> &Self::Collection {
        self
    }

    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self
    }
}
