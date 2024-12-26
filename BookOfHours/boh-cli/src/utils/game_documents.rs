use crate::model::aspected_items::AspectedItems;
use crate::model::aspects::Aspects;
use crate::model::consider_books::ConsiderBooks;
use crate::model::game_documents::GameDocuments;
use crate::model::lessons::Lessons;
use crate::model::recipe::Recipes;
use crate::model::save::{Autosave, StickyPayload, TentacledPayload};
use crate::model::skills::Skills;
use crate::model::tomes::Tomes;
use crate::model::{FindById, Identifiable, Mastery};
use crate::{deserialize_json_with_arbitrary_encoding, get_autosave_file};
use std::path::PathBuf;

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
        let recipes = GameDocuments::load_recipes(&recipes_dir_path)?;

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
        save_file_path: &PathBuf,
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
        let recipes = GameDocuments::load_recipes(&recipes_dir_path)?;

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

    pub(crate) fn check_if_item_manifested_fuzzy<T>(&self, game_item: &T) -> anyhow::Result<bool>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.check_if_item_manifested_fuzzy(game_item)
    }

    pub(crate) fn check_if_recipe_unlocked<T>(&self, game_item: &T) -> anyhow::Result<bool>
    where
        T: Identifiable + Clone + std::fmt::Debug,
    {
        self.autosave.check_if_recipe_unlocked(game_item)
    }

    pub(crate) fn check_if_item_manifested<T>(&self, game_item: &T) -> anyhow::Result<bool>
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

    pub(crate) fn check_if_tome_mastered<T>(&self, game_item: &T) -> bool
    where
        T: Mastery + Clone + std::fmt::Debug,
    {
        game_item.has_mastery()
    }

    #[tracing::instrument(skip(recipes_dir_path))]
    pub(crate) fn load_recipes(recipes_dir_path: &PathBuf) -> anyhow::Result<serde_json::Value> {
        let mut all_recipes: Recipes = Recipes { elements: vec![] };
        for recipe_file_name in crate::model::recipe::RECIPE_FILES {
            let recipe_file_path = recipes_dir_path.join(recipe_file_name);

            // Read the JSON contents of the file as an instance of `Recipe`.
            let recipes_data: serde_json::Value =
                deserialize_json_with_arbitrary_encoding(&recipe_file_path)?;

            let recipes: Recipes = serde_json_path_to_error::from_value(recipes_data)?;
            all_recipes.get_collection_mut().extend(recipes);
        }

        Ok(all_recipes.into())
    }
}
