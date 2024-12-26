use crate::model::aspected_items::AspectedItems;
use crate::model::aspects::Aspects;
use crate::model::consider_books::ConsiderBooks;
use crate::model::lessons::Lessons;
use crate::model::recipe::Recipes;
use crate::model::save::Autosave;
use crate::model::skills::Skills;
use crate::model::tomes::Tomes;

#[derive(Clone)]
pub(crate) struct GameDocuments {
    pub(crate) aspects: Aspects,
    pub(crate) aspected_items: AspectedItems,
    pub(crate) tomes: Tomes,
    pub(crate) consider_books: ConsiderBooks,
    pub(crate) skills: Skills,
    pub(crate) lessons: Lessons,
    //contamination_aspects: dyn GameCollection<QueryType::ContaminationAspects>,
    pub(crate) recipes: Recipes,
    pub(crate) autosave: Autosave,
}
