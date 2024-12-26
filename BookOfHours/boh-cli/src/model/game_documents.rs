use std::fs::File;
use std::io::BufReader;
use crate::{deserialize_json_with_arbitrary_encoding, get_autosave_file, init_json_data_explicit_save_file, read_config};
use crate::model::aspected_items::AspectedItems;
use crate::model::aspects::Aspects;
use crate::model::consider_books::ConsiderBooks;
use crate::model::lessons::Lessons;
use crate::model::save::{Autosave, StickyPayload, TentacledPayload};
use crate::model::skills::Skills;
use crate::model::tomes::Tomes;
use crate::model::{FindById, Identifiable, Mastery};
use std::path::PathBuf;
use tracing::trace;
use crate::model::recipe::Recipes;

static RECIPE_FILES: &'static [&'static str] = &[
    "__debug.json",
    "_backstops.json",
    "_collections.json",
    "_intercepts.json",
    "_legacy_crafting_4a_prenticeplus_ambittable_unfriendly.json",
    "_legacy_crafting_obsolete.json",
    // "_legacy.json",
    "_startup_recipes.json",
    "0_consider_decontaminations.json",
    "1_consider_books.json",
    "2a_consider_open.json",
    "2b_consider_generic.json",
    "2c_consider_resolve.json",
    "beasts.json",
    "bookbinding.json",
    "celestial_recipes_time.json",
    "celestial_recipes_weather.json",
    "correspondence_ordering.json",
    "correspondence.json",
    "crafting_0_numina.json",
    "crafting_0_rest.json",
    "crafting_1_chandlery.json",
    "crafting_1_evolutions.json",
    "crafting_1_simplemanipulations.json",
    "crafting_2_keeper.json",
    "crafting_3_scholar.json",
    "crafting_4b_prentice.json",
    "DLC_HOL_1_lighthouse.json",
    "DLC_HOL_1_meddling.json",
    "DLC_HOL_1a_salons_inauguration_end.json",
    "DLC_HOL_1b_salons_inauguration_ally_foe.json",
    "DLC_HOL_1b_salons_inauguration_mission.json",
    "DLC_HOL_2a_salons_blockers_basic.json",
    "DLC_HOL_2b_salons_blockers_beverages.json",
    "DLC_HOL_2c_salons_blockers_repasts.json",
    "DLC_HOL_3_salons_prototypes.json",
    "DLC_HOL_3b_salons_routing.json",
    "DLC_HOL_cooking.json",
    "DLC_HOL_correspondence_summoning.json",
    "DLC_HOL_gathering_2_seasonal.json",
    "DLC_HOL_interactions_specific.json",
    "DLC_HOL_leiter.json",
    "DLC_HOL_manuscripting_soph_improve.json",
    "DLC_HOL_manuscripting_soph_set.json",
    "DLC_HOL_manuscripting_write.json",
    "DLC_HOL_nrs.json",
    "DLC_HOL_patch_resurrect_incidents.json",
    "DLC_HOL_prototype_recipes.json",
    "DLC_HOL_salon_responses.json",
    "DLC_HOL_slnbk_languages.json",
    "DLC_HOL_slnbk_skillls.json",
    "DLC_HOL_slnbk_skills.json",
    "DLC_HOL_understanding_specific.json",
    "DLC_HOL_village_invitations.json",
    "gathering_1_exceptional.json",
    "gathering_2_seasonal.json",
    "other_activities.json",
    "renounce.json",
    "talk_1_visitors.json",
    "talk_2_visitors_payments_tutoring.json",
    "talk_3a_visitors_intercepts.json",
    "talk_3b_visitors_cantread.json",
    "talk_4a_visitors_intros.json",
    "talk_4b_visitors_specific_incidents.json",
    "talk_5_visitors_generic_consultations.json",
    "talk_5z_visitors_fallthrough_hints.json",
    "talk_6_assistance.json",
    "terrain.json",
    "understanding_1_numa.json",
    "understanding_2_upskill.json",
    "village_interactions.json",
    "visitors_correspondence_auctions.json",
    "wisdom_commitments_exotic.json",
    "wisdom_commitments.json",
    "z_fallthrough_hints.json",
    "z_fallthrough_placeholders.json",
];

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

impl GameDocuments {
    /// Constructs a new instance of the GameDocuments struct
    ///
    /// # Arguments
    ///
    /// * `aspects`: Aspects (e.g. "Knock")
    /// * `aspected_items`: Aspected Items (e.g. "Librarian's Glasses")
    /// * `tomes`: Tomes (e.g. "Exorcism for Girls")
    /// * `consider_books`:Consider Books (e.g. "I'm reading" and "I've read")
    /// * `skills`: Skills (e.g. "Inks of Power")
    ///
    /// returns: GameDocuments
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub(crate) fn new(
        aspects: Aspects,
        aspected_items: AspectedItems,
        tomes: Tomes,
        consider_books: ConsiderBooks,
        skills: Skills,
        lessons: Lessons,
        //contamination_aspects: Aspects,
        recipes: Recipes,
        autosave: Autosave,
    ) -> Self {
        GameDocuments {
            aspects,
            aspected_items,
            tomes,
            consider_books,
            skills,
            lessons,
            recipes,
            autosave,
        }
    }

    /// Using the provided game data directory path, parse and load game documents for use
    /// by the main application
    ///
    /// # Arguments
    ///
    /// * `path`: the path to the `core` directory of the exported game data
    ///
    /// returns: Result<GameDocuments, Error>
    ///
    /// # Examples
    ///
    /// ```
    /// let path: &PathBuf = "path_to_core_directory".into();
    /// let game_documents = GameDocuments::new_using_path(path)?;
    /// ```
    pub(crate) fn new_using_data_path(path: &PathBuf) -> anyhow::Result<Self> {
        let tomes_path = path.join("elements").join("tomes.json");
        let tomes_data = crate::deserialize_json_with_arbitrary_encoding(&tomes_path)?;
        let tomes = tomes_data.into();

        let aspected_items_path = path.join("elements").join("aspecteditems.json");
        let aspected_items_data =
            crate::deserialize_json_with_arbitrary_encoding(&aspected_items_path)?;

        // let contamination_aspects_path = path.join("elements").join( "contamination_aspects.json");
        //let contamination_aspects_data = deserialize_json_with_arbitrary_encoding(&contamination_aspects_path)?;

        let skills_path = path.join("elements").join("skills.json");
        let skills_data = crate::deserialize_json_with_arbitrary_encoding(&skills_path)?;

        let aspects_path = path.join("elements").join("_aspects.json");
        let aspects_data = crate::deserialize_json_with_arbitrary_encoding(&aspects_path)?;

        let consider_books_path = path.join("recipes").join("1_consider_books.json");
        let consider_books_data =
            crate::deserialize_json_with_arbitrary_encoding(&consider_books_path)?;

        let lessons_path = path.join("elements").join("xlessons.json");
        let lessons_data = crate::deserialize_json_with_arbitrary_encoding(&lessons_path)?;

        let recipes_dir_path = path.join("recipes");
        let recipes = load_recipes(&recipes_dir_path)?;

        let autosave_path = get_autosave_file()?;
        let autosave_data = crate::load_autosave(autosave_path)?;

        let game_documents = GameDocuments::new(
            aspects_data.into(),
            aspected_items_data.into(),
            tomes,
            consider_books_data.into(),
            skills_data.into(),
            lessons_data.into(),
            //contamination_aspects_data.into(),
            recipes.into(),
            autosave_data,
        );
        Ok(game_documents)
    }

    /// Using the provided game data directory path, parse and load game documents for use
    /// by the main application
    ///
    /// # Arguments
    ///
    /// * `path`: the path to the `core` directory of the exported game data
    ///
    /// returns: Result<GameDocuments, Error>
    ///
    /// # Examples
    ///
    /// ```
    /// let path: &PathBuf = "path_to_core_directory".into();
    /// let game_documents = GameDocuments::new_using_path(path)?;
    /// ```
    pub(crate) fn new_using_data_path_explicit_save_file(
        path: &PathBuf,
        save_file_path: &PathBuf
    ) -> anyhow::Result<Self> {
        let tomes_path = path.join("elements").join("tomes.json");
        let tomes_data = crate::deserialize_json_with_arbitrary_encoding(&tomes_path)?;
        let tomes = tomes_data.into();

        let aspected_items_path = path.join("elements").join("aspecteditems.json");
        let aspected_items_data =
            crate::deserialize_json_with_arbitrary_encoding(&aspected_items_path)?;

        // let contamination_aspects_path = path.join("elements").join( "contamination_aspects.json");
        //let contamination_aspects_data = deserialize_json_with_arbitrary_encoding(&contamination_aspects_path)?;

        let skills_path = path.join("elements").join("skills.json");
        let skills_data = crate::deserialize_json_with_arbitrary_encoding(&skills_path)?;

        let aspects_path = path.join("elements").join("_aspects.json");
        let aspects_data = crate::deserialize_json_with_arbitrary_encoding(&aspects_path)?;

        let consider_books_path = path.join("recipes").join("1_consider_books.json");
        let consider_books_data =
            crate::deserialize_json_with_arbitrary_encoding(&consider_books_path)?;

        let lessons_path = path.join("elements").join("xlessons.json");
        let lessons_data = crate::deserialize_json_with_arbitrary_encoding(&lessons_path)?;

        let recipes_dir_path = path.join("recipes");
        let recipes = load_recipes(&recipes_dir_path)?;

        let autosave_data = crate::load_autosave(save_file_path.into())?;

        let game_documents = GameDocuments::new(
            aspects_data.into(),
            aspected_items_data.into(),
            tomes,
            consider_books_data.into(),
            skills_data.into(),
            lessons_data.into(),
            //contamination_aspects_data.into(),
            recipes.into(),
            autosave_data,
        );
        Ok(game_documents)
    }

    pub(crate) fn check_if_item_manifested_fuzzy<T>(
        &self,
        game_item: &T
    ) -> anyhow::Result<bool>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.check_if_item_manifested_fuzzy(game_item)
    }

    pub(crate) fn check_if_recipe_unlocked<T>(
        &self,
        game_item: &T
    ) -> anyhow::Result<bool>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.check_if_recipe_unlocked(game_item)
    }

    pub(crate) fn check_if_item_manifested<T>(
        &self,
        game_item: &T
    ) -> anyhow::Result<bool>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.check_if_item_manifested(game_item)
    }

    pub(crate) fn get_tome_from_save_file<T>(
        &self,
        game_item: &T,
    ) -> anyhow::Result<TentacledPayload>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.get_item_from_save_file(game_item)
    }

    pub(crate) fn get_studying_item_from_save_file<T>(
        &self,
        game_item: &T,
    ) -> anyhow::Result<StickyPayload>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.get_studying_item_from_save_file(game_item)
    }

    pub(crate) fn check_if_tome_mastered<T>(
        &self,
        game_item: &T
    ) -> bool
    where
        T: Mastery + Clone + std::fmt::Debug,
    {
        game_item.has_mastery()
    }
}

#[tracing::instrument(skip(recipes_dir_path))]
fn load_recipes(recipes_dir_path: &PathBuf) -> anyhow::Result<serde_json::Value> {
    let mut all_recipes: Recipes = Recipes { elements: vec![] };
    for recipe_file_name in RECIPE_FILES {
        let recipe_file_path = recipes_dir_path.join(recipe_file_name);

        // Read the JSON contents of the file as an instance of `Recipe`.
        let recipes_data: serde_json::Value = deserialize_json_with_arbitrary_encoding(&recipe_file_path)?;

        let recipes: Recipes = serde_json_path_to_error::from_value(recipes_data)?;
        all_recipes.get_collection_mut().extend(recipes);
    }

    dbg!();
    Ok(all_recipes.into())
}
