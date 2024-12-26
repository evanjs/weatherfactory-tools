use crate::model::lessons::Lessons;
use serde_json::Value;
use tracing::{trace, warn};

impl Lessons {
    #[tracing::instrument(skip(self))]
    pub(crate) fn get_lesson_string(&self, id: &str) -> Option<String> {
        if let Some(elements) = &self.elements {
            Some(
                elements
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
                    .unwrap_or_else(|| format!("No lesson found for id {}", id)),
            )
        } else {
            warn!("No lessons found for ID: {id}");
            None
        }
    }
}

impl From<Value> for Lessons {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
