use crate::model::save::StickyPayload;
use crate::model::{Identifiable, Mastery};

impl Mastery for StickyPayload {
    fn has_mastery(&self) -> bool {
        let mutations = &self.mutations;
        let mutations_mastered = [
            mutations.mastery_grail,
            mutations.mastery_heart,
            mutations.mastery_knock,
            mutations.mastery_lantern,
            mutations.mastery_moon,
            mutations.mastery_moth,
            mutations.mastery_nectar,
            mutations.mastery_rose,
            mutations.mastery_scale,
            mutations.mastery_sky,
            mutations.mastery_winter];

        mutations_mastered
            .iter()
            .any(|x| x.is_some_and(|val| val > 0))
    }
}

impl Identifiable for StickyPayload {
    fn id(&self) -> &str {
        &self.id
    }

    fn inner_id(&self) -> &str {
        &self.entity_id
    }
}
