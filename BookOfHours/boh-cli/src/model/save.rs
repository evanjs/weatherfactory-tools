use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Autosave {
    #[serde(rename = "$type")]
    pub(crate) autosave_type: Option<String>,
    pub(crate) character_creation_commands: Option<Vec<CharacterCreationCommand>>,
    pub(crate) root_population_command: Option<RootPopulationCommand>,
    pub(crate) populate_xamanek_command: Option<PopulateXamanekCommand>,
    pub(crate) notification_commands: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) version: Option<Version>,
    pub(crate) is_fresh: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterCreationCommand {
    #[serde(rename = "$type")]
    pub(crate) character_creation_command_type: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) profession: Option<String>,
    pub(crate) active_legacy_id: Option<String>,
    pub(crate) ending_triggered_id: Option<serde_json::Value>,
    pub(crate) date_time_created: Option<String>,
    pub(crate) in_progress_history_records: Option<CurrentItineraries>,
    pub(crate) previous_character_history_records: Option<PreviousCharacterHistoryRecords>,
    pub(crate) unique_elements_manifested: Option<Vec<String>>,
    pub(crate) ambittable_recipes_unlocked: Option<Vec<String>>,
    pub(crate) created_in_version: Option<Version>,
    pub(crate) current_focus: Option<CurrentFocus>,
    pub(crate) current_houses: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    #[serde(rename = "$type")]
    pub(crate) version_type: Option<String>,
    pub(crate) version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CurrentFocus {
    #[serde(rename = "$type")]
    pub(crate) current_focus_type: Option<String>,
    pub(crate) x: Option<f64>,
    pub(crate) y: Option<f64>,
    pub(crate) z: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentItineraries {
    #[serde(rename = "$type")]
    pub(crate) current_itineraries_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousCharacterHistoryRecords {
    #[serde(rename = "$type")]
    pub(crate) previous_character_history_records_type: Option<String>,
    pub(crate) lastcharactername: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PopulateXamanekCommand {
    #[serde(rename = "$type")]
    pub(crate) populate_xamanek_command_type: Option<String>,
    pub(crate) current_itineraries: Option<CurrentItineraries>,
    pub(crate) current_enviro_fx_commands: Option<CurrentEnviroFxCommands>,
    pub(crate) current_sphere_blocks: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CurrentEnviroFxCommands {
    #[serde(rename = "$type")]
    pub(crate) current_enviro_fx_commands_type: Option<String>,
    pub(crate) vignette: Option<Atrium>,
    pub(crate) sky: Option<Atrium>,
    pub(crate) weather: Option<Atrium>,
    pub(crate) music: Option<Atrium>,
    pub(crate) brancrug: Option<Atrium>,
    pub(crate) season: Option<Atrium>,
    pub(crate) meta: Option<Atrium>,
    pub(crate) cucurbitbridge: Option<Atrium>,
    pub(crate) lodge: Option<Atrium>,
    pub(crate) secondbeach: Option<Atrium>,
    pub(crate) gatehouse: Option<Atrium>,
    pub(crate) watchmanstower1: Option<Atrium>,
    pub(crate) watchmanstower2: Option<Atrium>,
    pub(crate) cloister: Option<Atrium>,
    pub(crate) practicgarden: Option<Atrium>,
    pub(crate) longtower1: Option<Atrium>,
    pub(crate) longtower2: Option<Atrium>,
    pub(crate) entrancehall: Option<Atrium>,
    pub(crate) scentgarden: Option<Atrium>,
    #[serde(rename = "grandascent.g")]
    pub(crate) grandascent_g: Option<Atrium>,
    #[serde(rename = "grandascent.1")]
    pub(crate) grandascent_1: Option<Atrium>,
    #[serde(rename = "grandascent.2")]
    pub(crate) grandascent_2: Option<Atrium>,
    pub(crate) readingroom: Option<Atrium>,
    pub(crate) infirmaryg: Option<Atrium>,
    pub(crate) mazarineroom: Option<Atrium>,
    #[serde(rename = "infirmary1b")]
    pub(crate) infirmary1_b: Option<Atrium>,
    pub(crate) motleytower1: Option<Atrium>,
    pub(crate) westcottroom: Option<Atrium>,
    pub(crate) physicgarden: Option<Atrium>,
    pub(crate) refectory: Option<Atrium>,
    #[serde(rename = "grandascent.3")]
    pub(crate) grandascent_3: Option<Atrium>,
    pub(crate) atrium: Option<Atrium>,
    pub(crate) fluddgallery: Option<Atrium>,
    pub(crate) nave: Option<Atrium>,
    pub(crate) severnchamber: Option<Atrium>,
    pub(crate) violetchamber: Option<Atrium>,
    #[serde(rename = "infirmary1a")]
    pub(crate) infirmary1_a: Option<Atrium>,
    pub(crate) mapchamber: Option<Atrium>,
    pub(crate) palechamber: Option<Atrium>,
    pub(crate) motleytower2: Option<Atrium>,
    pub(crate) servantshall: Option<Atrium>,
    pub(crate) librarianquarters: Option<Atrium>,
    pub(crate) stores: Option<Atrium>,
    pub(crate) backstairs: Option<Atrium>,
    pub(crate) vestibulumtransitus: Option<Atrium>,
    pub(crate) kitchen: Option<Atrium>,
    pub(crate) foundry: Option<Atrium>,
    pub(crate) gullscryloggia: Option<Atrium>,
    pub(crate) motleytower3: Option<Atrium>,
    pub(crate) pantry: Option<Atrium>,
    pub(crate) gloriousstair: Option<Atrium>,
    pub(crate) windlitgallery: Option<Atrium>,
    pub(crate) well: Option<Atrium>,
    pub(crate) radiantstair: Option<Atrium>,
    pub(crate) churchtower1: Option<Atrium>,
    pub(crate) chapterhouse: Option<Atrium>,
    pub(crate) solarium: Option<Atrium>,
    pub(crate) welldescent: Option<Atrium>,
    pub(crate) deepwelldescent: Option<Atrium>,
    pub(crate) kitchengarden: Option<Atrium>,
    pub(crate) ballroomduellinghall: Option<Atrium>,
    pub(crate) nightgallery: Option<Atrium>,
    pub(crate) gaolbridge: Option<Atrium>,
    pub(crate) churchtower2: Option<Atrium>,
    pub(crate) servantsquarters2: Option<Atrium>,
    pub(crate) chancel: Option<Atrium>,
    pub(crate) servantsquarters1: Option<Atrium>,
    pub(crate) gaolyard: Option<Atrium>,
    pub(crate) staff_room: Option<Atrium>,
    pub(crate) gaolhall: Option<Atrium>,
    pub(crate) sacredspring: Option<Atrium>,
    pub(crate) winecellar: Option<Atrium>,
    pub(crate) hallofmirrors: Option<Atrium>,
    pub(crate) gaolkitchen: Option<Atrium>,
    pub(crate) stairtenebrous: Option<Atrium>,
    pub(crate) earthcellar: Option<Atrium>,
    pub(crate) cell_adept: Option<Atrium>,
    #[serde(rename = "earlbrian'sfield")]
    pub(crate) earlbrian_sfield: Option<Atrium>,
    pub(crate) gullcolony: Option<Atrium>,
    pub(crate) hermitcell: Option<Atrium>,
    pub(crate) gullscrytower1: Option<Atrium>,
    #[serde(rename = "smuggler'sden")]
    pub(crate) smuggler_sden: Option<Atrium>,
    pub(crate) ourladybeneath: Option<Atrium>,
    pub(crate) gaolcellar: Option<Atrium>,
    pub(crate) bells: Option<Atrium>,
    pub(crate) spire: Option<Atrium>,
    pub(crate) gullscrytower2: Option<Atrium>,
    pub(crate) loadingdock: Option<Atrium>,
    pub(crate) gullscrytower3: Option<Atrium>,
    pub(crate) upperpumproom: Option<Atrium>,
    pub(crate) oubliette: Option<Atrium>,
    pub(crate) boathouse: Option<Atrium>,
    pub(crate) rowenarium: Option<Atrium>,
    pub(crate) whisperingstair: Option<Atrium>,
    pub(crate) hiddenstair: Option<Atrium>,
    pub(crate) crucibletower1: Option<Atrium>,
    pub(crate) salon: Option<Atrium>,
    pub(crate) run: Option<Atrium>,
    pub(crate) crucibletower2: Option<Atrium>,
    pub(crate) lowerpumproom: Option<Atrium>,
    pub(crate) seacaves: Option<Atrium>,
    pub(crate) hallofvoices: Option<Atrium>,
    pub(crate) crucibletower3: Option<Atrium>,
    pub(crate) silvervault: Option<Atrium>,
    pub(crate) columbiccrypt: Option<Atrium>,
    pub(crate) catacombs: Option<Atrium>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Atrium {
    #[serde(rename = "$type")]
    pub(crate) atrium_type: Option<AtriumType>,
    pub(crate) concern: Option<String>,
    pub(crate) effect: Option<Effect>,
    pub(crate) parameter: Option<String>,
    pub(crate) get_full_fx: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AtriumType {
    #[serde(rename = "SecretHistories.Commands.EnviroFxCommand, SecretHistories.Main")]
    SecretHistoriesCommandsEnviroFxCommandSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    Day,
    Morning,
    None,
    Open,
    Setspeed,
    Summer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootPopulationCommand {
    #[serde(rename = "$type")]
    pub(crate) root_population_command_type: Option<String>,
    pub(crate) mutations: Option<RootPopulationCommandMutations>,
    pub(crate) spheres: Option<Vec<RootPopulationCommandSphere>>,
    pub(crate) dealers_table: Option<DealersTable>,
    pub(crate) tokens_at_arbitrary_paths: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealersTable {
    #[serde(rename = "$type")]
    pub(crate) dealers_table_type: Option<DealersTableType>,
    pub(crate) identifier: Option<String>,
    pub(crate) spheres: Option<Vec<DealersTableSphere>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DealersTableType {
    #[serde(rename = "PopulateDominionCommand")]
    PopulateDominionCommand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealersTableSphere {
    #[serde(rename = "$type")]
    pub(crate) sphere_type: Option<SphereType>,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: Option<PurpleGoverningSphereSpec>,
    pub(crate) tokens: Option<Vec<PurpleToken>>,
    pub(crate) shrouded: Option<bool>,
    pub(crate) persistent_sphere_data: Option<serde_json::Value>,
    pub(crate) illuminations: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: Option<GoverningSphereSpecType>,
    pub(crate) label: Option<String>,
    pub(crate) action_id: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) available_from_house: Option<String>,
    pub(crate) essential: Option<CurrentItineraries>,
    pub(crate) required: Option<CurrentItineraries>,
    pub(crate) forbidden: Option<CurrentItineraries>,
    pub(crate) if_aspects_present: Option<CurrentItineraries>,
    pub(crate) greedy: Option<bool>,
    pub(crate) angels: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) from_path: Option<Path>,
    pub(crate) en_route_sphere_path: Option<Path>,
    pub(crate) windows_sphere_path: Option<Path>,
    pub(crate) sphere_type: Option<String>,
    pub(crate) allow_any_token: Option<bool>,
    pub(crate) id: Option<String>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Path {
    #[serde(rename = "$type")]
    pub(crate) path_type: Option<EnRouteSpherePathType>,
    pub(crate) filter: Option<serde_json::Value>,
    pub(crate) path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnRouteSpherePathType {
    #[serde(rename = "SecretHistories.Fucine.FucinePath, SecretHistories.Main")]
    SecretHistoriesFucineFucinePathSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GoverningSphereSpecType {
    #[serde(rename = "SecretHistories.Entities.SphereSpec, SecretHistories.Main")]
    SecretHistoriesEntitiesSphereSpecSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SphereType {
    #[serde(rename = "SphereCreationCommand")]
    SphereCreationCommand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleToken {
    #[serde(rename = "$type")]
    pub(crate) token_type: Option<TokenType>,
    pub(crate) location: Option<Location>,
    pub(crate) home_location: Option<serde_json::Value>,
    pub(crate) payload: Option<PurplePayload>,
    pub(crate) placement_already_chronicled: Option<bool>,
    pub(crate) defunct: Option<bool>,
    pub(crate) current_state: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    #[serde(rename = "$type")]
    pub(crate) location_type: Option<HomeLocationType>,
    pub(crate) local_position: Box<Option<LocalPosition>>,
    pub(crate) at_sphere_path: Option<Path>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPosition {
    #[serde(rename = "$type")]
    pub(crate) local_position_type: Option<LocalPositionType>,
    pub(crate) x: Option<f64>,
    pub(crate) y: Option<f64>,
    pub(crate) z: Option<f64>,
    pub(crate) magnitude: Option<f64>,
    pub(crate) sqr_magnitude: Option<f64>,
    pub(crate) normalized: Box<Option<LocalPosition>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalPositionType {
    #[serde(rename = "UnityEngine.Vector3, UnityEngine.CoreModule")]
    UnityEngineVector3UnityEngineCoreModule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HomeLocationType {
    #[serde(rename = "SecretHistories.UI.TokenLocation, SecretHistories.Main")]
    SecretHistoriesUiTokenLocationSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurplePayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: Option<PayloadType>,
    pub(crate) id: Option<String>,
    pub(crate) entity_id: Option<String>,
    pub(crate) quantity: Option<i64>,
    pub(crate) mutations: Option<CurrentItineraries>,
    pub(crate) illuminations: Option<CurrentItineraries>,
    pub(crate) defunct: Option<bool>,
    pub(crate) is_shrouded: Option<bool>,
    pub(crate) lifetime_remaining: Option<f64>,
    pub(crate) dominions: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PayloadType {
    #[serde(rename = "ElementStackCreationCommand")]
    ElementStackCreationCommand,
    #[serde(rename = "PopulateNxCommand")]
    PopulateNxCommand,
    #[serde(rename = "PopulateTerrainFeatureCommand")]
    PopulateTerrainFeatureCommand,
    #[serde(rename = "SituationCreationCommand")]
    SituationCreationCommand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "TokenCreationCommand")]
    TokenCreationCommand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RootPopulationCommandMutations {
    #[serde(rename = "$type")]
    pub(crate) mutations_type: Option<String>,
    pub(crate) ii: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootPopulationCommandSphere {
    #[serde(rename = "$type")]
    pub(crate) sphere_type: Option<SphereType>,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: Option<FluffyGoverningSphereSpec>,
    pub(crate) tokens: Option<Vec<FluffyToken>>,
    pub(crate) shrouded: Option<bool>,
    pub(crate) persistent_sphere_data: Option<PersistentSphereData>,
    pub(crate) illuminations: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: Option<GoverningSphereSpecType>,
    pub(crate) label: Option<PurpleLabel>,
    pub(crate) action_id: Option<String>,
    pub(crate) description: Option<Description>,
    pub(crate) available_from_house: Option<String>,
    pub(crate) essential: Option<PurpleEssential>,
    pub(crate) required: Option<PurpleRequired>,
    pub(crate) forbidden: Option<PurpleForbidden>,
    pub(crate) if_aspects_present: Option<CurrentItineraries>,
    pub(crate) greedy: Option<bool>,
    pub(crate) angels: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) from_path: Option<Path>,
    pub(crate) en_route_sphere_path: Option<Path>,
    pub(crate) windows_sphere_path: Option<Path>,
    pub(crate) sphere_type: Option<String>,
    pub(crate) allow_any_token: Option<bool>,
    pub(crate) id: Option<String>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "Clouds mask the sky...")]
    CloudsMaskTheSky,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "[Keep things here until you tidy them away.]")]
    KeepThingsHereUntilYouTidyThemAway,
    #[serde(rename = "#UI_ROOMINPUT_DESCRIPTION#")]
    UiRoominputDescription,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleEssential {
    #[serde(rename = "$type")]
    pub(crate) essential_type: Option<EssentialType>,
    pub(crate) assistance: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EssentialType {
    #[serde(rename = "SecretHistories.Core.AspectsDictionary, SecretHistories.Main")]
    SecretHistoriesCoreAspectsDictionarySecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleForbidden {
    #[serde(rename = "$type")]
    pub(crate) forbidden_type: Option<EssentialType>,
    pub(crate) memory: Option<i64>,
    pub(crate) ability: Option<i64>,
    pub(crate) skill: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PurpleLabel {
    Belongings,
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "SEASONS")]
    Seasons,
    #[serde(rename = "#UI_ROOMINPUT_LABEL#")]
    UiRoominputLabel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleRequired {
    #[serde(rename = "$type")]
    pub(crate) required_type: Option<EssentialType>,
    pub(crate) ability: Option<i64>,
    pub(crate) skill: Option<i64>,
    pub(crate) memory: Option<i64>,
    pub(crate) lesson: Option<i64>,
    pub(crate) circumstance: Option<i64>,
    pub(crate) scale: Option<i64>,
    pub(crate) knock: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PersistentSphereData {
    #[serde(rename = "$type")]
    pub(crate) persistent_sphere_data_type: Option<String>,
    pub(crate) default_position: Option<f64>,
    pub(crate) current_position: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyToken {
    #[serde(rename = "$type")]
    pub(crate) token_type: Option<TokenType>,
    pub(crate) location: Option<Location>,
    pub(crate) home_location: Option<Location>,
    pub(crate) payload: Option<FluffyPayload>,
    pub(crate) placement_already_chronicled: Option<bool>,
    pub(crate) defunct: Option<bool>,
    pub(crate) current_state: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyPayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: PayloadType,
    pub(crate) id: Option<String>,
    pub(crate) dominions: Option<Vec<PurpleDominion>>,
    pub(crate) mutations: Option<TentacledMutations>,
    pub(crate) edens_enacted: Option<Vec<EdensEnacted>>,
    pub(crate) is_sealed: Option<bool>,
    pub(crate) is_shrouded: Option<bool>,
    pub(crate) quantity: Option<i64>,
    pub(crate) has_previously_unshrouded: Option<bool>,
    pub(crate) last_situation_created: Option<serde_json::Value>,
    pub(crate) verb_id: Option<String>,
    pub(crate) output_path: Option<String>,
    pub(crate) current_recipe_id: Option<String>,
    pub(crate) fallback_recipe_id: Option<String>,
    pub(crate) state_identifier: Option<i64>,
    pub(crate) time_remaining: Option<f64>,
    pub(crate) is_open: Option<bool>,
    pub(crate) has_ghost_note: Option<bool>,
    pub(crate) command_queue: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) entity_id: Option<String>,
    pub(crate) illuminations: Option<CurrentItineraries>,
    pub(crate) defunct: Option<bool>,
    pub(crate) lifetime_remaining: Option<f64>,
    pub(crate) outcome_message: Option<String>,
    pub(crate) locked_in_recipe: Option<LockedInRecipe>,
    pub(crate) last_run_recipe: Option<LastRunRecipe>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleDominion {
    #[serde(rename = "$type")]
    pub(crate) dominion_type: Option<DealersTableType>,
    pub(crate) identifier: Option<Identifier>,
    pub(crate) spheres: Option<Vec<PurpleSphere>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Identifier {
    #[serde(rename = "ContentsDominion")]
    ContentsDominion,
    Notes,
    Output,
    #[serde(rename = "RecipeThresholds")]
    RecipeThresholds,
    #[serde(rename = "RoomContentsDominion")]
    RoomContentsDominion,
    #[serde(rename = "SealedDominion")]
    SealedDominion,
    #[serde(rename = "ShroudedDominion")]
    ShroudedDominion,
    Storage,
    #[serde(rename = "UnshroudedDominion")]
    UnshroudedDominion,
    #[serde(rename = "VerbThresholds")]
    VerbThresholds,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleSphere {
    #[serde(rename = "$type")]
    pub(crate) sphere_type: Option<SphereType>,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: Option<TentacledGoverningSphereSpec>,
    pub(crate) tokens: Option<Vec<TentacledToken>>,
    pub(crate) shrouded: Option<bool>,
    pub(crate) persistent_sphere_data: Option<serde_json::Value>,
    pub(crate) illuminations: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: Option<GoverningSphereSpecType>,
    pub(crate) label: Option<FluffyLabel>,
    pub(crate) action_id: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) available_from_house: Option<AvailableFromHouse>,
    pub(crate) essential: Option<FluffyEssential>,
    pub(crate) required: Option<FluffyRequired>,
    pub(crate) forbidden: Option<FluffyForbidden>,
    pub(crate) if_aspects_present: Option<CurrentItineraries>,
    pub(crate) greedy: Option<bool>,
    pub(crate) angels: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) from_path: Option<Path>,
    pub(crate) en_route_sphere_path: Option<Path>,
    pub(crate) windows_sphere_path: Option<Path>,
    pub(crate) sphere_type: Option<String>,
    pub(crate) allow_any_token: Option<bool>,
    pub(crate) id: Option<String>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailableFromHouse {
    Advent1,
    Advent10,
    Advent12,
    Advent14,
    Advent17,
    Advent19,
    Advent2,
    Advent21,
    Advent23,
    Advent25,
    Advent4,
    Advent6,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "HouseOfLight")]
    HouseOfLight,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyEssential {
    #[serde(rename = "$type")]
    pub(crate) essential_type: Option<EssentialType>,
    pub(crate) skill: Option<i64>,
    pub(crate) finished: Option<i64>,
    #[serde(rename = "incident.box")]
    pub(crate) incident_box: Option<i64>,
    #[serde(rename = "incident.revolution")]
    pub(crate) incident_revolution: Option<i64>,
    #[serde(rename = "incident.mob")]
    pub(crate) incident_mob: Option<i64>,
    #[serde(rename = "incident.hunt.changing")]
    pub(crate) incident_hunt_changing: Option<i64>,
    #[serde(rename = "incident.mystical")]
    pub(crate) incident_mystical: Option<i64>,
    #[serde(rename = "incident.omen.dawn")]
    pub(crate) incident_omen_dawn: Option<i64>,
    #[serde(rename = "incident.project.solar")]
    pub(crate) incident_project_solar: Option<i64>,
    #[serde(rename = "incident.heist")]
    pub(crate) incident_heist: Option<i64>,
    #[serde(rename = "incident.stalk")]
    pub(crate) incident_stalk: Option<i64>,
    #[serde(rename = "incident.project.ingenious")]
    pub(crate) incident_project_ingenious: Option<i64>,
    #[serde(rename = "incident.rising")]
    pub(crate) incident_rising: Option<i64>,
    #[serde(rename = "incident.omen.horizon")]
    pub(crate) incident_omen_horizon: Option<i64>,
    #[serde(rename = "incident.opera.wings")]
    pub(crate) incident_opera_wings: Option<i64>,
    #[serde(rename = "incident.lost.find")]
    pub(crate) incident_lost_find: Option<i64>,
    #[serde(rename = "incident.opera.apollo")]
    pub(crate) incident_opera_apollo: Option<i64>,
    #[serde(rename = "incident.rite.awakening")]
    pub(crate) incident_rite_awakening: Option<i64>,
    #[serde(rename = "incident.pan")]
    pub(crate) incident_pan: Option<i64>,
    #[serde(rename = "incident.rite.renewal")]
    pub(crate) incident_rite_renewal: Option<i64>,
    #[serde(rename = "incident.curse")]
    pub(crate) incident_curse: Option<i64>,
    #[serde(rename = "incident.intrusion")]
    pub(crate) incident_intrusion: Option<i64>,
    #[serde(rename = "incident.wound")]
    pub(crate) incident_wound: Option<i64>,
    #[serde(rename = "incident.observe.scaly")]
    pub(crate) incident_observe_scaly: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyForbidden {
    #[serde(rename = "$type")]
    pub(crate) forbidden_type: Option<EssentialType>,
    pub(crate) fixed: Option<i64>,
    #[serde(rename = "record.phonograph")]
    pub(crate) record_phonograph: Option<i64>,
    pub(crate) film: Option<i64>,
    pub(crate) uncatalogued: Option<i64>,
    pub(crate) soaked: Option<i64>,
    #[serde(rename = "wisdom.committed")]
    pub(crate) wisdom_committed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FluffyLabel {
    #[serde(rename = "ABANDONED AT THE THRESHOLD")]
    AbandonedAtTheThreshold,
    #[serde(rename = "ABANDONED AT THE WATER'S EDGE")]
    AbandonedAtTheWaterSEdge,
    #[serde(rename = "BOOKSHELF")]
    Bookshelf,
    #[serde(rename = "COMFORT")]
    Comfort,
    Consider,
    #[serde(rename = "Eighth Librarian of the Curia of the Isle")]
    EighthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Eleventh Librarian of the Curia of the Isle")]
    EleventhLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "EXPLORING THE TREE")]
    ExploringTheTree,
    #[serde(rename = "Fifth Librarian of the Curia of the Isle")]
    FifthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "First Librarian of the Curia of the Isle")]
    FirstLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Fourth Librarian of the Curia of the Isle")]
    FourthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "--")]
    Label,
    #[serde(rename = "Ninth Librarian of the Curia of the Isle")]
    NinthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Reverend Timothy's Christmas Tree")]
    ReverendTimothySChristmasTree,
    #[serde(rename = "Second Librarian of the Curia of the Isle")]
    SecondLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Seventh Librarian of the Curia of the Isle")]
    SeventhLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Sixth Librarian of the Curia of the Isle")]
    SixthLibrarianOfTheCuriaOfTheIsle,
    Talk,
    #[serde(rename = "Tenth Librarian of the Curia of the Isle")]
    TenthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "1st Baron Brancrug")]
    The1StBaronBrancrug,
    #[serde(rename = "2nd Baron Brancrug")]
    The2NdBaronBrancrug,
    #[serde(rename = "3rd Baron Brancrug")]
    The3RdBaronBrancrug,
    #[serde(rename = "4th Baron Brancrug")]
    The4ThBaronBrancrug,
    #[serde(rename = "5th Baron Brancrug")]
    The5ThBaronBrancrug,
    #[serde(rename = "6th Baron Brancrug")]
    The6ThBaronBrancrug,
    #[serde(rename = "7th Baroness Brancrug")]
    The7ThBaronessBrancrug,
    #[serde(rename = "THE HEART OF THE")]
    TheHeartOfThe,
    #[serde(rename = "THINGS")]
    Things,
    #[serde(rename = "Third Librarian of the Curia of the Isle")]
    ThirdLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Thirteenth Librarian of the Curia of the Isle?")]
    ThirteenthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "Twelfth Librarian of the Curia of the Isle")]
    TwelfthLibrarianOfTheCuriaOfTheIsle,
    #[serde(rename = "#UI_NX_SLOT0_TITLE#")]
    UiNxSlot0Title,
    #[serde(rename = "#UI_WT_MEMORYLOCUS_INPUT_LABEL#")]
    UiWtMemorylocusInputLabel,
    #[serde(rename = "WALLART")]
    Wallart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyRequired {
    #[serde(rename = "$type")]
    pub(crate) required_type: Option<EssentialType>,
    pub(crate) thing: Option<i64>,
    pub(crate) wallart: Option<i64>,
    pub(crate) comfort: Option<i64>,
    pub(crate) cooperative: Option<i64>,
    pub(crate) beast: Option<i64>,
    pub(crate) cache: Option<i64>,
    pub(crate) lesson: Option<i64>,
    pub(crate) skill: Option<i64>,
    pub(crate) readable: Option<i64>,
    pub(crate) considerable: Option<i64>,
    pub(crate) visitor: Option<i64>,
    pub(crate) employable: Option<i64>,
    pub(crate) assistance: Option<i64>,
    #[serde(rename = "cost.tally")]
    pub(crate) cost_tally: Option<i64>,
    #[serde(rename = "visitor.embarking")]
    pub(crate) visitor_embarking: Option<i64>,
    pub(crate) journal: Option<i64>,
    #[serde(rename = "w.preservation")]
    pub(crate) w_preservation: Option<i64>,
    #[serde(rename = "w.bosk")]
    pub(crate) w_bosk: Option<i64>,
    #[serde(rename = "w.birdsong")]
    pub(crate) w_birdsong: Option<i64>,
    #[serde(rename = "w.horomachistry")]
    pub(crate) w_horomachistry: Option<i64>,
    #[serde(rename = "w.ithastry")]
    pub(crate) w_ithastry: Option<i64>,
    #[serde(rename = "w.illumination")]
    pub(crate) w_illumination: Option<i64>,
    #[serde(rename = "w.skolekosophy")]
    pub(crate) w_skolekosophy: Option<i64>,
    #[serde(rename = "w.hushery")]
    pub(crate) w_hushery: Option<i64>,
    #[serde(rename = "w.nyctodromy")]
    pub(crate) w_nyctodromy: Option<i64>,
    pub(crate) memory: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledToken {
    #[serde(rename = "$type")]
    pub(crate) token_type: Option<TokenType>,
    pub(crate) location: Option<Location>,
    pub(crate) home_location: Option<Location>,
    pub(crate) payload: Option<TentacledPayload>,
    pub(crate) placement_already_chronicled: Option<bool>,
    pub(crate) defunct: Option<bool>,
    pub(crate) current_state: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledPayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: Option<PayloadType>,
    pub(crate) last_situation_created: Option<serde_json::Value>,
    pub(crate) id: Option<String>,
    pub(crate) verb_id: Option<String>,
    pub(crate) output_path: Option<serde_json::Value>,
    pub(crate) current_recipe_id: Option<Id>,
    pub(crate) fallback_recipe_id: Option<Id>,
    pub(crate) quantity: Option<i64>,
    pub(crate) state_identifier: Option<i64>,
    pub(crate) time_remaining: Option<f64>,
    pub(crate) mutations: Option<FluffyMutations>,
    pub(crate) is_open: Option<bool>,
    pub(crate) dominions: Option<Vec<FluffyDominion>>,
    pub(crate) has_ghost_note: Option<bool>,
    pub(crate) command_queue: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) entity_id: Option<String>,
    pub(crate) illuminations: Option<Illuminations>,
    pub(crate) defunct: Option<bool>,
    pub(crate) is_shrouded: Option<bool>,
    pub(crate) lifetime_remaining: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Id {
    #[serde(rename = "craft.scholar.inks.containment_flower_liquid.solomon_winter")]
    CraftScholarInksContainmentFlowerLiquidSolomonWinter,
    #[serde(rename = "craft.scholar.rhyme.remembrance_remains_essential.periost_winter")]
    CraftScholarRhymeRemembranceRemainsEssentialPeriostWinter,
    #[serde(rename = "NullRecipe")]
    NullRecipe,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyDominion {
    #[serde(rename = "$type")]
    pub(crate) dominion_type: Option<DealersTableType>,
    pub(crate) identifier: Option<Identifier>,
    pub(crate) spheres: Option<Vec<FluffySphere>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffySphere {
    #[serde(rename = "$type")]
    pub(crate) sphere_type: Option<SphereType>,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: Option<StickyGoverningSphereSpec>,
    pub(crate) tokens: Option<Vec<StickyToken>>,
    pub(crate) shrouded: Option<bool>,
    pub(crate) persistent_sphere_data: Option<serde_json::Value>,
    pub(crate) illuminations: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: Option<GoverningSphereSpecType>,
    pub(crate) label: Option<String>,
    pub(crate) action_id: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) available_from_house: Option<String>,
    pub(crate) essential: Option<TentacledEssential>,
    pub(crate) required: Option<TentacledRequired>,
    pub(crate) forbidden: Option<TentacledForbidden>,
    pub(crate) if_aspects_present: Option<CurrentItineraries>,
    pub(crate) greedy: Option<bool>,
    pub(crate) angels: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) from_path: Option<Path>,
    pub(crate) en_route_sphere_path: Option<Path>,
    pub(crate) windows_sphere_path: Option<Path>,
    pub(crate) sphere_type: Option<String>,
    pub(crate) allow_any_token: Option<bool>,
    pub(crate) id: Option<String>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledEssential {
    #[serde(rename = "$type")]
    pub(crate) essential_type: Option<EssentialType>,
    pub(crate) ability: Option<i64>,
    pub(crate) skill: Option<i64>,
    pub(crate) campable: Option<i64>,
    pub(crate) distributable: Option<i64>,
    pub(crate) thing: Option<i64>,
    pub(crate) memory: Option<i64>,
    pub(crate) heart: Option<i64>,
    pub(crate) fuel: Option<i64>,
    pub(crate) liquid: Option<i64>,
    pub(crate) metal: Option<i64>,
    pub(crate) inspired: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledForbidden {
    #[serde(rename = "$type")]
    pub(crate) forbidden_type: Option<EssentialType>,
    pub(crate) fatigued: Option<i64>,
    pub(crate) malady: Option<i64>,
    pub(crate) numatic: Option<i64>,
    pub(crate) film: Option<i64>,
    #[serde(rename = "record.phonograph")]
    pub(crate) record_phonograph: Option<i64>,
    #[serde(rename = "visitor.villager")]
    pub(crate) visitor_villager: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledRequired {
    #[serde(rename = "$type")]
    pub(crate) required_type: Option<EssentialType>,
    pub(crate) knock: Option<i64>,
    pub(crate) lantern: Option<i64>,
    pub(crate) heart: Option<i64>,
    pub(crate) forge: Option<i64>,
    pub(crate) edge: Option<i64>,
    pub(crate) grail: Option<i64>,
    pub(crate) winter: Option<i64>,
    pub(crate) tally: Option<i64>,
    #[serde(rename = "post.ready")]
    pub(crate) post_ready: Option<i64>,
    pub(crate) pence: Option<i64>,
    pub(crate) ability: Option<i64>,
    pub(crate) soaked: Option<i64>,
    pub(crate) soiled: Option<i64>,
    pub(crate) scale: Option<i64>,
    pub(crate) rose: Option<i64>,
    pub(crate) thing: Option<i64>,
    pub(crate) beast: Option<i64>,
    pub(crate) moon: Option<i64>,
    #[serde(rename = "interest.heart")]
    pub(crate) interest_heart: Option<i64>,
    pub(crate) blank: Option<i64>,
    pub(crate) readable: Option<i64>,
    pub(crate) ink: Option<i64>,
    pub(crate) tool: Option<i64>,
    pub(crate) sky: Option<i64>,
    pub(crate) lens: Option<i64>,
    pub(crate) memory: Option<i64>,
    pub(crate) nectar: Option<i64>,
    #[serde(rename = "interest.winter")]
    pub(crate) interest_winter: Option<i64>,
    #[serde(rename = "visitor.villager")]
    pub(crate) visitor_villager: Option<i64>,
    pub(crate) visitor: Option<i64>,
    #[serde(rename = "appointment.persistent")]
    pub(crate) appointment_persistent: Option<i64>,
    pub(crate) film: Option<i64>,
    pub(crate) moth: Option<i64>,
    pub(crate) remains: Option<i64>,
    pub(crate) flower: Option<i64>,
    pub(crate) liquid: Option<i64>,
    pub(crate) fuel: Option<i64>,
    #[serde(rename = "interest.moth")]
    pub(crate) interest_moth: Option<i64>,
    pub(crate) light: Option<i64>,
    #[serde(rename = "interest.grail")]
    pub(crate) interest_grail: Option<i64>,
    #[serde(rename = "interest.knock")]
    pub(crate) interest_knock: Option<i64>,
    #[serde(rename = "interest.sky")]
    pub(crate) interest_sky: Option<i64>,
    pub(crate) fatigued: Option<i64>,
    pub(crate) malady: Option<i64>,
    pub(crate) beverage: Option<i64>,
    pub(crate) restorative: Option<i64>,
    pub(crate) covenant: Option<i64>,
    pub(crate) omen: Option<i64>,
    pub(crate) sound: Option<i64>,
    pub(crate) correspondence: Option<i64>,
    pub(crate) fabric: Option<i64>,
    pub(crate) sustenance: Option<i64>,
    pub(crate) ingredient: Option<i64>,
    pub(crate) root: Option<i64>,
    pub(crate) leaf: Option<i64>,
    pub(crate) knife: Option<i64>,
    pub(crate) kitchenware: Option<i64>,
    pub(crate) stone: Option<i64>,
    pub(crate) metal: Option<i64>,
    pub(crate) glass: Option<i64>,
    pub(crate) penny: Option<i64>,
    pub(crate) journal: Option<i64>,
    pub(crate) device: Option<i64>,
    pub(crate) pigment: Option<i64>,
    pub(crate) wood: Option<i64>,
    pub(crate) cooperative: Option<i64>,
    pub(crate) ductile: Option<i64>,
    pub(crate) woven: Option<i64>,
    pub(crate) wooden: Option<i64>,
    pub(crate) mark: Option<i64>,
    pub(crate) gem: Option<i64>,
    #[serde(rename = "hidden.mihail.altar")]
    pub(crate) hidden_mihail_altar: Option<i64>,
    #[serde(rename = "interest.scale")]
    pub(crate) interest_scale: Option<i64>,
    #[serde(rename = "record.phonograph")]
    pub(crate) record_phonograph: Option<i64>,
    #[serde(rename = "interest.moon")]
    pub(crate) interest_moon: Option<i64>,
    pub(crate) key: Option<i64>,
    pub(crate) material: Option<i64>,
    #[serde(rename = "hidden.mihail.sarcophagus")]
    pub(crate) hidden_mihail_sarcophagus: Option<i64>,
    pub(crate) hokobald: Option<i64>,
    pub(crate) dagmar: Option<i64>,
    pub(crate) coquille: Option<i64>,
    pub(crate) fraser: Option<i64>,
    pub(crate) stanislav: Option<i64>,
    pub(crate) ehsan: Option<i64>,
    #[serde(rename = "suitable.secretary.salvant.surprise")]
    pub(crate) suitable_secretary_salvant_surprise: Option<i64>,
    pub(crate) agdistis: Option<i64>,
    pub(crate) arthur: Option<i64>,
    pub(crate) azita: Option<i64>,
    pub(crate) douglas: Option<i64>,
    pub(crate) chaima: Option<i64>,
    pub(crate) zachary: Option<i64>,
    pub(crate) yvette: Option<i64>,
    pub(crate) corso: Option<i64>,
    pub(crate) arun: Option<i64>,
    #[serde(rename = "suitable.secretary.nunciant.surprise")]
    pub(crate) suitable_secretary_nunciant_surprise: Option<i64>,
    pub(crate) connie: Option<i64>,
    pub(crate) aladim: Option<i64>,
    pub(crate) olympe: Option<i64>,
    pub(crate) serena: Option<i64>,
    #[serde(rename = "suitable.secretary.persistent.surprise")]
    pub(crate) suitable_secretary_persistent_surprise: Option<i64>,
    pub(crate) morgen: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyToken {
    #[serde(rename = "$type")]
    pub(crate) token_type: Option<TokenType>,
    pub(crate) location: Option<Location>,
    pub(crate) home_location: Option<Location>,
    pub(crate) payload: Option<StickyPayload>,
    pub(crate) placement_already_chronicled: Option<bool>,
    pub(crate) defunct: Option<bool>,
    pub(crate) current_state: Option<CurrentItineraries>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyPayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: Option<PayloadType>,
    pub(crate) id: Option<String>,
    pub(crate) entity_id: Option<EntityId>,
    pub(crate) quantity: Option<i64>,
    pub(crate) mutations: Option<PurpleMutations>,
    pub(crate) illuminations: Option<Illuminations>,
    pub(crate) defunct: Option<bool>,
    pub(crate) is_shrouded: Option<bool>,
    pub(crate) lifetime_remaining: Option<f64>,
    pub(crate) dominions: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityId {
    #[serde(rename = "lenten.rose")]
    LentenRose,
    #[serde(rename = "s.inks.containment")]
    SInksContainment,
    #[serde(rename = "s.rhyme.remembrance")]
    SRhymeRemembrance,
    #[serde(rename = "scrapings.phosphorescent")]
    ScrapingsPhosphorescent,
    #[serde(rename = "tlg.note")]
    TlgNote,
    #[serde(rename = "wormwood.dream")]
    WormwoodDream,
    Xsha,
    Xwis3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Illuminations {
    #[serde(rename = "$type")]
    pub(crate) illuminations_type: Option<String>,
    #[serde(rename = "tlg.notes.title")]
    pub(crate) tlg_notes_title: Option<String>,
    #[serde(rename = "tlg.notes.description")]
    pub(crate) tlg_notes_description: Option<String>,
    #[serde(rename = "tlg.notes.emphasislevel")]
    pub(crate) tlg_notes_emphasislevel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleMutations {
    #[serde(rename = "$type")]
    pub(crate) mutations_type: Option<String>,
    #[serde(rename = "wisdom.committed")]
    pub(crate) wisdom_committed: Option<i64>,
    #[serde(rename = "w.skolekosophy")]
    pub(crate) w_skolekosophy: Option<i64>,
    pub(crate) winter: Option<i64>,
    pub(crate) moon: Option<i64>,
    pub(crate) skill: Option<i64>,
    #[serde(rename = "a.xhea")]
    pub(crate) a_xhea: Option<i64>,
    #[serde(rename = "w.horomachistry")]
    pub(crate) w_horomachistry: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyMutations {
    #[serde(rename = "$type")]
    pub(crate) mutations_type: Option<String>,
    #[serde(rename = "mastery.heart")]
    pub(crate) mastery_heart: Option<i64>,
    #[serde(rename = "mastery.moon")]
    pub(crate) mastery_moon: Option<i64>,
    #[serde(rename = "tx.arun")]
    pub(crate) tx_arun: Option<i64>,
    #[serde(rename = "tx.auntmopsy")]
    pub(crate) tx_auntmopsy: Option<i64>,
    #[serde(rename = "tx.julian")]
    pub(crate) tx_julian: Option<i64>,
    #[serde(rename = "tx.connie")]
    pub(crate) tx_connie: Option<i64>,
    #[serde(rename = "mastery.moth")]
    pub(crate) mastery_moth: Option<i64>,
    #[serde(rename = "mastery.edge")]
    pub(crate) mastery_edge: Option<i64>,
    #[serde(rename = "tx.arthur")]
    pub(crate) tx_arthur: Option<i64>,
    #[serde(rename = "mastery.forge")]
    pub(crate) mastery_forge: Option<i64>,
    #[serde(rename = "mastery.grail")]
    pub(crate) mastery_grail: Option<i64>,
    #[serde(rename = "mastery.lantern")]
    pub(crate) mastery_lantern: Option<i64>,
    #[serde(rename = "mastery.sky")]
    pub(crate) mastery_sky: Option<i64>,
    #[serde(rename = "mastery.nectar")]
    pub(crate) mastery_nectar: Option<i64>,
    #[serde(rename = "mastery.scale")]
    pub(crate) mastery_scale: Option<i64>,
    #[serde(rename = "tx.fraser")]
    pub(crate) tx_fraser: Option<i64>,
    #[serde(rename = "tx.azita")]
    pub(crate) tx_azita: Option<i64>,
    #[serde(rename = "mystery.grail")]
    pub(crate) mystery_grail: Option<i64>,
    pub(crate) soph: Option<i64>,
    #[serde(rename = "mystery.winter")]
    pub(crate) mystery_winter: Option<i64>,
    pub(crate) knock: Option<i64>,
    #[serde(rename = "mastery.winter")]
    pub(crate) mastery_winter: Option<i64>,
    #[serde(rename = "tx.chaima")]
    pub(crate) tx_chaima: Option<i64>,
    #[serde(rename = "mastery.rose")]
    pub(crate) mastery_rose: Option<i64>,
    #[serde(rename = "tx.franklin")]
    pub(crate) tx_franklin: Option<i64>,
    #[serde(rename = "tx.dagmar")]
    pub(crate) tx_dagmar: Option<i64>,
    #[serde(rename = "mastery.knock")]
    pub(crate) mastery_knock: Option<i64>,
    #[serde(rename = "tx.corso")]
    pub(crate) tx_corso: Option<i64>,
    #[serde(rename = "tx.serena")]
    pub(crate) tx_serena: Option<i64>,
    #[serde(rename = "tx.douglas")]
    pub(crate) tx_douglas: Option<i64>,
    #[serde(rename = "tx.aladim")]
    pub(crate) tx_aladim: Option<i64>,
    #[serde(rename = "tx.yvette")]
    pub(crate) tx_yvette: Option<i64>,
    #[serde(rename = "tx.zachary")]
    pub(crate) tx_zachary: Option<i64>,
    #[serde(rename = "tx.agdistis")]
    pub(crate) tx_agdistis: Option<i64>,
    #[serde(rename = "tx.ehsan")]
    pub(crate) tx_ehsan: Option<i64>,
    #[serde(rename = "tx.stanislav")]
    pub(crate) tx_stanislav: Option<i64>,
    #[serde(rename = "tx.coquille")]
    pub(crate) tx_coquille: Option<i64>,
    #[serde(rename = "tx.hokobald")]
    pub(crate) tx_hokobald: Option<i64>,
    #[serde(rename = "tx.olympe")]
    pub(crate) tx_olympe: Option<i64>,
    #[serde(rename = "address.dagmar")]
    pub(crate) address_dagmar: Option<i64>,
    #[serde(rename = "address.chaima")]
    pub(crate) address_chaima: Option<i64>,
    #[serde(rename = "address.fraser")]
    pub(crate) address_fraser: Option<i64>,
    #[serde(rename = "address.azita")]
    pub(crate) address_azita: Option<i64>,
    #[serde(rename = "address.arthur")]
    pub(crate) address_arthur: Option<i64>,
    #[serde(rename = "acted.arthur.revolution.edge")]
    pub(crate) acted_arthur_revolution_edge: Option<i64>,
    #[serde(rename = "acted.fraser.project.solar.lantern")]
    pub(crate) acted_fraser_project_solar_lantern: Option<i64>,
    #[serde(rename = "acted.douglas.project.solar.lantern")]
    pub(crate) acted_douglas_project_solar_lantern: Option<i64>,
    #[serde(rename = "acted.arthur.project.solar.lantern")]
    pub(crate) acted_arthur_project_solar_lantern: Option<i64>,
    #[serde(rename = "acted.olympe.project.solar.forge")]
    pub(crate) acted_olympe_project_solar_forge: Option<i64>,
    #[serde(rename = "acted.serena.project.solar.lantern")]
    pub(crate) acted_serena_project_solar_lantern: Option<i64>,
    #[serde(rename = "acted.hokobald.project.solar.forge")]
    pub(crate) acted_hokobald_project_solar_forge: Option<i64>,
    #[serde(rename = "acted.corso.project.solar.lantern")]
    pub(crate) acted_corso_project_solar_lantern: Option<i64>,
    #[serde(rename = "acted.chaima.stalk.rose")]
    pub(crate) acted_chaima_stalk_rose: Option<i64>,
    #[serde(rename = "acted.dagmar.stalk.rose")]
    pub(crate) acted_dagmar_stalk_rose: Option<i64>,
    #[serde(rename = "acted.azita.stalk.edge")]
    pub(crate) acted_azita_stalk_edge: Option<i64>,
    #[serde(rename = "acted.aladim.stalk.rose")]
    pub(crate) acted_aladim_stalk_rose: Option<i64>,
    #[serde(rename = "acted.arthur.stalk.edge")]
    pub(crate) acted_arthur_stalk_edge: Option<i64>,
    #[serde(rename = "acted.dagmar.omen.horizon.rose")]
    pub(crate) acted_dagmar_omen_horizon_rose: Option<i64>,
    #[serde(rename = "acted.aladim.omen.horizon.rose")]
    pub(crate) acted_aladim_omen_horizon_rose: Option<i64>,
    #[serde(rename = "acted.arun.lost.find.moth")]
    pub(crate) acted_arun_lost_find_moth: Option<i64>,
    #[serde(rename = "acted.dagmar.lost.find.rose")]
    pub(crate) acted_dagmar_lost_find_rose: Option<i64>,
    #[serde(rename = "acted.chaima.lost.find.rose")]
    pub(crate) acted_chaima_lost_find_rose: Option<i64>,
    #[serde(rename = "acted.agdistis.pan.heart")]
    pub(crate) acted_agdistis_pan_heart: Option<i64>,
    #[serde(rename = "acted.ehsan.pan.heart")]
    pub(crate) acted_ehsan_pan_heart: Option<i64>,
    #[serde(rename = "acted.douglas.pan.heart")]
    pub(crate) acted_douglas_pan_heart: Option<i64>,
    #[serde(rename = "acted.stanislav.rite.renewal.nectar")]
    pub(crate) acted_stanislav_rite_renewal_nectar: Option<i64>,
    #[serde(rename = "acted.arun.rite.renewal.moon")]
    pub(crate) acted_arun_rite_renewal_moon: Option<i64>,
    #[serde(rename = "acted.connie.rite.renewal.moon")]
    pub(crate) acted_connie_rite_renewal_moon: Option<i64>,
    #[serde(rename = "acted.coquille.intrusion.knock")]
    pub(crate) acted_coquille_intrusion_knock: Option<i64>,
    #[serde(rename = "acted.yvette.intrusion.winter")]
    pub(crate) acted_yvette_intrusion_winter: Option<i64>,
    #[serde(rename = "acted.dagmar.intrusion.knock")]
    pub(crate) acted_dagmar_intrusion_knock: Option<i64>,
    #[serde(rename = "acted.agdistis.wound.heart")]
    pub(crate) acted_agdistis_wound_heart: Option<i64>,
    #[serde(rename = "acted.yvette.wound.winter")]
    pub(crate) acted_yvette_wound_winter: Option<i64>,
    #[serde(rename = "acted.zachary.wound.winter")]
    pub(crate) acted_zachary_wound_winter: Option<i64>,
    #[serde(rename = "acted.douglas.wound.heart")]
    pub(crate) acted_douglas_wound_heart: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdensEnacted {
    Advent,
    #[serde(rename = "HouseOfLight")]
    HouseOfLight,
    #[serde(rename = "patch_ehsan")]
    PatchEhsan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LastRunRecipe {
    #[serde(rename = "$type")]
    pub(crate) last_run_recipe_type: Option<LastRunRecipeType>,
    pub(crate) priority: Option<i64>,
    pub(crate) action_id: Option<ActionId>,
    pub(crate) blocks: Option<bool>,
    pub(crate) reqs: Option<Reqs>,
    pub(crate) extant_reqs: Option<CurrentItineraries>,
    pub(crate) greq: Option<CurrentItineraries>,
    pub(crate) ngreq: Option<CurrentItineraries>,
    #[serde(rename = "FXReqs")]
    pub(crate) fx_reqs: Option<CurrentItineraries>,
    pub(crate) seeking: Option<CurrentItineraries>,
    pub(crate) effects: Option<Effects>,
    pub(crate) x_pans: Option<CurrentItineraries>,
    #[serde(rename = "FX")]
    pub(crate) fx: Option<CurrentItineraries>,
    pub(crate) aspects: Option<Aspects>,
    pub(crate) mutations: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) purge: Option<CurrentItineraries>,
    pub(crate) halt_verb: Option<CurrentItineraries>,
    pub(crate) delete_verb: Option<CurrentItineraries>,
    pub(crate) achievements: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) signal_important_loop: Option<bool>,
    pub(crate) audio_one_shot: Option<serde_json::Value>,
    pub(crate) signal_ending_flavour: Option<i64>,
    pub(crate) craftable: Option<bool>,
    pub(crate) notable: Option<bool>,
    pub(crate) hint_only: Option<bool>,
    pub(crate) ambittable: Option<bool>,
    pub(crate) warmup: Option<i64>,
    pub(crate) inherits: Option<String>,
    pub(crate) preface: Option<String>,
    pub(crate) start_label: Option<StartLabel>,
    pub(crate) label: Option<Label>,
    pub(crate) start_description: Option<Label>,
    pub(crate) desc: Option<String>,
    pub(crate) comments: Option<String>,
    pub(crate) deck_effects: Option<CurrentItineraries>,
    pub(crate) alt: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) lalt: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) inductions: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) linked: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) ending: Option<String>,
    pub(crate) icon: Option<Icon>,
    pub(crate) burn_image: Option<serde_json::Value>,
    pub(crate) run: Option<String>,
    pub(crate) pre_slots: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) slots: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) internal_deck: Option<InternalDeck>,
    pub(crate) id: Option<Id>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionId {
    Nx,
    X,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aspects {
    #[serde(rename = "$type")]
    pub(crate) aspects_type: Option<EssentialType>,
    #[serde(rename = "nx.helping")]
    pub(crate) nx_helping: Option<i64>,
    #[serde(rename = "inspiring.fear.nowhere")]
    pub(crate) inspiring_fear_nowhere: Option<i64>,
    #[serde(rename = "nx.opening")]
    pub(crate) nx_opening: Option<i64>,
    #[serde(rename = "inspiring.agenda.corrivality")]
    pub(crate) inspiring_agenda_corrivality: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effects {
    #[serde(rename = "$type")]
    pub(crate) effects_type: Option<String>,
    #[serde(rename = "spencer.enroute.out")]
    pub(crate) spencer_enroute_out: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Icon {
    Arthur,
    Coquille,
    X,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InternalDeck {
    #[serde(rename = "$type")]
    pub(crate) internal_deck_type: Option<InternalDeckType>,
    pub(crate) default_card: Option<String>,
    pub(crate) reset_on_exhaustion: Option<bool>,
    pub(crate) label: Option<String>,
    pub(crate) desc: Option<String>,
    pub(crate) cover: Option<Cover>,
    pub(crate) comments: Option<String>,
    pub(crate) is_hidden: Option<bool>,
    pub(crate) draws: Option<i64>,
    pub(crate) spec: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) draw_messages: Option<CurrentItineraries>,
    pub(crate) default_draw_messages: Option<CurrentItineraries>,
    pub(crate) id: Option<InternalDeckId>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cover {
    Books,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InternalDeckId {
    #[serde(rename = "DeckSpec")]
    DeckSpec,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InternalDeckType {
    #[serde(rename = "SecretHistories.Entities.DeckSpec, SecretHistories.Main")]
    SecretHistoriesEntitiesDeckSpecSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Ask Coquille")]
    AskCoquille,
    #[serde(rename = ".")]
    Empty,
    #[serde(rename = "The Revolution?")]
    TheRevolution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LastRunRecipeType {
    #[serde(rename = "SecretHistories.Entities.NullRecipe, SecretHistories.Main")]
    SecretHistoriesEntitiesNullRecipeSecretHistoriesMain,
    #[serde(rename = "SecretHistories.Entities.Recipe, SecretHistories.Main")]
    SecretHistoriesEntitiesRecipeSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reqs {
    #[serde(rename = "$type")]
    pub(crate) reqs_type: Option<String>,
    #[serde(rename = "acted.chaima.revolution.edge")]
    pub(crate) acted_chaima_revolution_edge: Option<String>,
    pub(crate) arthur: Option<String>,
    #[serde(rename = "n.revolution")]
    pub(crate) n_revolution: Option<String>,
    #[serde(rename = "acted.azita.revolution.edge")]
    pub(crate) acted_azita_revolution_edge: Option<String>,
    #[serde(rename = "n.guest")]
    pub(crate) n_guest: Option<String>,
    #[serde(rename = "acted.coquille.intrusion.knock")]
    pub(crate) acted_coquille_intrusion_knock: Option<String>,
    pub(crate) coquille: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartLabel {
    Arthur,
    Coquille,
    #[serde(rename = ".")]
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LockedInRecipe {
    #[serde(rename = "$type")]
    pub(crate) locked_in_recipe_type: Option<LastRunRecipeType>,
    pub(crate) priority: Option<i64>,
    pub(crate) action_id: Option<ActionId>,
    pub(crate) blocks: Option<bool>,
    pub(crate) reqs: Option<CurrentItineraries>,
    pub(crate) extant_reqs: Option<CurrentItineraries>,
    pub(crate) greq: Option<CurrentItineraries>,
    pub(crate) ngreq: Option<CurrentItineraries>,
    #[serde(rename = "FXReqs")]
    pub(crate) fx_reqs: Option<CurrentItineraries>,
    pub(crate) seeking: Option<CurrentItineraries>,
    pub(crate) effects: Option<CurrentItineraries>,
    pub(crate) x_pans: Option<CurrentItineraries>,
    #[serde(rename = "FX")]
    pub(crate) fx: Option<CurrentItineraries>,
    pub(crate) aspects: Option<CurrentItineraries>,
    pub(crate) mutations: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) purge: Option<CurrentItineraries>,
    pub(crate) halt_verb: Option<CurrentItineraries>,
    pub(crate) delete_verb: Option<CurrentItineraries>,
    pub(crate) achievements: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) signal_important_loop: Option<bool>,
    pub(crate) audio_one_shot: Option<serde_json::Value>,
    pub(crate) signal_ending_flavour: Option<i64>,
    pub(crate) craftable: Option<bool>,
    pub(crate) notable: Option<bool>,
    pub(crate) hint_only: Option<bool>,
    pub(crate) ambittable: Option<bool>,
    pub(crate) warmup: Option<i64>,
    pub(crate) inherits: Option<String>,
    pub(crate) preface: Option<Label>,
    pub(crate) start_label: Option<Label>,
    pub(crate) label: Option<Label>,
    pub(crate) start_description: Option<Label>,
    pub(crate) desc: Option<Label>,
    pub(crate) comments: Option<String>,
    pub(crate) deck_effects: Option<CurrentItineraries>,
    pub(crate) alt: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) lalt: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) inductions: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) linked: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) ending: Option<String>,
    pub(crate) icon: Option<ActionId>,
    pub(crate) burn_image: Option<serde_json::Value>,
    pub(crate) run: Option<String>,
    pub(crate) pre_slots: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) slots: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) internal_deck: Option<InternalDeck>,
    pub(crate) id: Option<Id>,
    pub(crate) lever: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TentacledMutations {
    #[serde(rename = "$type")]
    pub(crate) mutations_type: Option<String>,
    #[serde(rename = "v.numa.winter.a")]
    pub(crate) v_numa_winter_a: Option<i64>,
    #[serde(rename = "v.numa.moon.b")]
    pub(crate) v_numa_moon_b: Option<i64>,
    #[serde(rename = "interest.moon")]
    pub(crate) interest_moon: Option<i64>,
    #[serde(rename = "v.rite.renewal")]
    pub(crate) v_rite_renewal: Option<i64>,
    #[serde(rename = "v.project.solar")]
    pub(crate) v_project_solar: Option<i64>,
    #[serde(rename = "interest.lantern")]
    pub(crate) interest_lantern: Option<i64>,
    #[serde(rename = "interest.forge")]
    pub(crate) interest_forge: Option<i64>,
    #[serde(rename = "v.numa.grail.b")]
    pub(crate) v_numa_grail_b: Option<i64>,
    #[serde(rename = "interest.grail")]
    pub(crate) interest_grail: Option<i64>,
    #[serde(rename = "v.numa.grail.a")]
    pub(crate) v_numa_grail_a: Option<i64>,
    #[serde(rename = "v.pan")]
    pub(crate) v_pan: Option<i64>,
    #[serde(rename = "interest.heart")]
    pub(crate) interest_heart: Option<i64>,
    #[serde(rename = "v.wound")]
    pub(crate) v_wound: Option<i64>,
    #[serde(rename = "v.intrusion")]
    pub(crate) v_intrusion: Option<i64>,
    #[serde(rename = "interest.winter")]
    pub(crate) interest_winter: Option<i64>,
    #[serde(rename = "interest.nectar")]
    pub(crate) interest_nectar: Option<i64>,
    #[serde(rename = "v.lost.find")]
    pub(crate) v_lost_find: Option<i64>,
    #[serde(rename = "interest.moth")]
    pub(crate) interest_moth: Option<i64>,
    pub(crate) grateful: Option<i64>,
    #[serde(rename = "v.numa.moon.a")]
    pub(crate) v_numa_moon_a: Option<i64>,
    #[serde(rename = "interest.knock")]
    pub(crate) interest_knock: Option<i64>,
    #[serde(rename = "v.stalk")]
    pub(crate) v_stalk: Option<i64>,
    #[serde(rename = "interest.rose")]
    pub(crate) interest_rose: Option<i64>,
    #[serde(rename = "v.omen.horizon")]
    pub(crate) v_omen_horizon: Option<i64>,
    #[serde(rename = "agenda.corrivality")]
    pub(crate) agenda_corrivality: Option<i64>,
    #[serde(rename = "fear.nowhere")]
    pub(crate) fear_nowhere: Option<i64>,
    #[serde(rename = "v.hunt.changing")]
    pub(crate) v_hunt_changing: Option<i64>,
    #[serde(rename = "interest.scale")]
    pub(crate) interest_scale: Option<i64>,
    #[serde(rename = "interest.edge")]
    pub(crate) interest_edge: Option<i64>,
    #[serde(rename = "v.revolution")]
    pub(crate) v_revolution: Option<i64>,
    pub(crate) moth: Option<i64>,
    pub(crate) finished: Option<i64>,
    #[serde(rename = "wisdom.committed")]
    pub(crate) wisdom_committed: Option<i64>,
    #[serde(rename = "w.bosk")]
    pub(crate) w_bosk: Option<i64>,
    #[serde(rename = "a.xhausted")]
    pub(crate) a_xhausted: Option<i64>,
    pub(crate) scale: Option<i64>,
    pub(crate) nectar: Option<i64>,
    pub(crate) skill: Option<i64>,
    pub(crate) sky: Option<i64>,
    pub(crate) rose: Option<i64>,
    #[serde(rename = "a.xfet")]
    pub(crate) a_xfet: Option<i64>,
    #[serde(rename = "w.birdsong")]
    pub(crate) w_birdsong: Option<i64>,
    pub(crate) moon: Option<i64>,
    pub(crate) heart: Option<i64>,
    #[serde(rename = "w.hushery")]
    pub(crate) w_hushery: Option<i64>,
    pub(crate) winter: Option<i64>,
    #[serde(rename = "w.ithastry")]
    pub(crate) w_ithastry: Option<i64>,
    pub(crate) lantern: Option<i64>,
    #[serde(rename = "w.illumination")]
    pub(crate) w_illumination: Option<i64>,
    pub(crate) knock: Option<i64>,
    pub(crate) forge: Option<i64>,
    #[serde(rename = "a.xcho")]
    pub(crate) a_xcho: Option<i64>,
    #[serde(rename = "a.xhea")]
    pub(crate) a_xhea: Option<i64>,
    #[serde(rename = "w.nyctodromy")]
    pub(crate) w_nyctodromy: Option<i64>,
    #[serde(rename = "a.xere")]
    pub(crate) a_xere: Option<i64>,
    #[serde(rename = "w.skolekosophy")]
    pub(crate) w_skolekosophy: Option<i64>,
    pub(crate) edge: Option<i64>,
    pub(crate) grail: Option<i64>,
    #[serde(rename = "w.preservation")]
    pub(crate) w_preservation: Option<i64>,
    #[serde(rename = "a.xtri")]
    pub(crate) a_xtri: Option<i64>,
    #[serde(rename = "w.horomachistry")]
    pub(crate) w_horomachistry: Option<i64>,
    #[serde(rename = "a.xsha")]
    pub(crate) a_xsha: Option<i64>,
    #[serde(rename = "a.xpho")]
    pub(crate) a_xpho: Option<i64>,
    #[serde(rename = "a.xmet")]
    pub(crate) a_xmet: Option<i64>,
    #[serde(rename = "acted.julian.numa.moon.b.moon")]
    pub(crate) acted_julian_numa_moon_b_moon: Option<i64>,
    #[serde(rename = "acted.franklin.numa.grail.b.grail")]
    pub(crate) acted_franklin_numa_grail_b_grail: Option<i64>,
    #[serde(rename = "acted.franklin.numa.grail.a.grail")]
    pub(crate) acted_franklin_numa_grail_a_grail: Option<i64>,
    #[serde(rename = "acted.auntmopsy.numa.moon.a.moon")]
    pub(crate) acted_auntmopsy_numa_moon_a_moon: Option<i64>,
    #[serde(rename = "orderplaced.bottle.armagnac")]
    pub(crate) orderplaced_bottle_armagnac: Option<i64>,
    #[serde(rename = "cost.pence")]
    pub(crate) cost_pence: Option<i64>,
    #[serde(rename = "post.ready")]
    pub(crate) post_ready: Option<i64>,
    #[serde(rename = "orderplaced.almonds")]
    pub(crate) orderplaced_almonds: Option<i64>,
    #[serde(rename = "orderplaced.bottle.lambig")]
    pub(crate) orderplaced_bottle_lambig: Option<i64>,
    #[serde(rename = "orderplaced.bottle.strathcoyne")]
    pub(crate) orderplaced_bottle_strathcoyne: Option<i64>,
    #[serde(rename = "acted.fraser.hunt.changing.scale")]
    pub(crate) acted_fraser_hunt_changing_scale: Option<i64>,
    #[serde(rename = "acted.azita.hunt.changing.edge")]
    pub(crate) acted_azita_hunt_changing_edge: Option<i64>,
    #[serde(rename = "acted.arthur.hunt.changing.edge")]
    pub(crate) acted_arthur_hunt_changing_edge: Option<i64>,
}
