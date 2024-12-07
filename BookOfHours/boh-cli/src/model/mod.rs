use std::fmt::Debug;
use tracing::{debug, trace, warn};

pub(crate) mod aspects;
pub(crate) mod skills;
pub(crate) mod tomes;

// Define a shared trait for elements that have an ID field
pub trait Identifiable {
    fn id(&self) -> &str;
}

// Define a trait for collections that can find elements by ID
pub trait FindById {
    type Item: Identifiable;

    type Collection: IntoIterator<Item = Self::Item>;

    fn get_collection(&self) -> &Self::Collection;

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
