use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Autosave {
    #[serde(rename = "$type")]
    pub(crate) autosave_type: String,
    pub(crate) character_creation_commands: Vec<CharacterCreationCommand>,
    pub(crate) root_population_command: RootPopulationCommand,
    pub(crate) populate_xamanek_command: PopulateXamanekCommand,
    pub(crate) notification_commands: Vec<Option<serde_json::Value>>,
    pub(crate) version: Version,
    pub(crate) is_fresh: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterCreationCommand {
    #[serde(rename = "$type")]
    pub(crate) character_creation_command_type: String,
    pub(crate) name: String,
    pub(crate) profession: String,
    pub(crate) active_legacy_id: String,
    pub(crate) ending_triggered_id: Option<serde_json::Value>,
    pub(crate) date_time_created: String,
    pub(crate) in_progress_history_records: CurrentItineraries,
    pub(crate) previous_character_history_records: PreviousCharacterHistoryRecords,
    pub(crate) unique_elements_manifested: Vec<String>,
    pub(crate) ambittable_recipes_unlocked: Vec<String>,
    pub(crate) created_in_version: Version,
    pub(crate) current_focus: CurrentFocus,
    pub(crate) current_houses: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    #[serde(rename = "$type")]
    pub(crate) version_type: String,
    pub(crate) version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CurrentFocus {
    #[serde(rename = "$type")]
    pub(crate) current_focus_type: String,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentItineraries {
    #[serde(rename = "$type")]
    pub(crate) current_itineraries_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviousCharacterHistoryRecords {
    #[serde(rename = "$type")]
    pub(crate) previous_character_history_records_type: String,
    pub(crate) lastcharactername: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PopulateXamanekCommand {
    #[serde(rename = "$type")]
    pub(crate) populate_xamanek_command_type: String,
    pub(crate) current_itineraries: CurrentItineraries,
    pub(crate) current_enviro_fx_commands: CurrentEnviroFxCommands,
    pub(crate) current_sphere_blocks: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CurrentEnviroFxCommands {
    #[serde(rename = "$type")]
    pub(crate) current_enviro_fx_commands_type: String,
    pub(crate) vignette: Atrium,
    pub(crate) sky: Atrium,
    pub(crate) weather: Atrium,
    pub(crate) music: Atrium,
    pub(crate) brancrug: Atrium,
    pub(crate) season: Atrium,
    pub(crate) meta: Atrium,
    pub(crate) cucurbitbridge: Atrium,
    pub(crate) lodge: Atrium,
    pub(crate) secondbeach: Atrium,
    pub(crate) gatehouse: Atrium,
    pub(crate) watchmanstower1: Atrium,
    pub(crate) watchmanstower2: Atrium,
    pub(crate) cloister: Atrium,
    pub(crate) practicgarden: Atrium,
    pub(crate) longtower1: Atrium,
    pub(crate) longtower2: Atrium,
    pub(crate) entrancehall: Atrium,
    pub(crate) scentgarden: Atrium,
    #[serde(rename = "grandascent.g")]
    pub(crate) grandascent_g: Atrium,
    #[serde(rename = "grandascent.1")]
    pub(crate) grandascent_1: Atrium,
    #[serde(rename = "grandascent.2")]
    pub(crate) grandascent_2: Atrium,
    pub(crate) readingroom: Atrium,
    pub(crate) infirmaryg: Atrium,
    pub(crate) mazarineroom: Atrium,
    #[serde(rename = "infirmary1b")]
    pub(crate) infirmary1_b: Atrium,
    pub(crate) motleytower1: Atrium,
    pub(crate) westcottroom: Atrium,
    pub(crate) physicgarden: Atrium,
    pub(crate) refectory: Atrium,
    #[serde(rename = "grandascent.3")]
    pub(crate) grandascent_3: Atrium,
    pub(crate) atrium: Atrium,
    pub(crate) fluddgallery: Atrium,
    pub(crate) nave: Atrium,
    pub(crate) severnchamber: Atrium,
    pub(crate) violetchamber: Atrium,
    #[serde(rename = "infirmary1a")]
    pub(crate) infirmary1_a: Atrium,
    pub(crate) mapchamber: Atrium,
    pub(crate) palechamber: Atrium,
    pub(crate) motleytower2: Atrium,
    pub(crate) servantshall: Atrium,
    pub(crate) librarianquarters: Atrium,
    pub(crate) stores: Atrium,
    pub(crate) backstairs: Atrium,
    pub(crate) vestibulumtransitus: Atrium,
    pub(crate) kitchen: Atrium,
    pub(crate) foundry: Atrium,
    pub(crate) gullscryloggia: Atrium,
    pub(crate) motleytower3: Atrium,
    pub(crate) pantry: Atrium,
    pub(crate) gloriousstair: Atrium,
    pub(crate) windlitgallery: Atrium,
    pub(crate) well: Atrium,
    pub(crate) radiantstair: Atrium,
    pub(crate) churchtower1: Atrium,
    pub(crate) chapterhouse: Atrium,
    pub(crate) solarium: Atrium,
    pub(crate) welldescent: Atrium,
    pub(crate) deepwelldescent: Atrium,
    pub(crate) kitchengarden: Atrium,
    pub(crate) ballroomduellinghall: Atrium,
    pub(crate) nightgallery: Atrium,
    pub(crate) gaolbridge: Atrium,
    pub(crate) churchtower2: Atrium,
    pub(crate) servantsquarters2: Atrium,
    pub(crate) chancel: Atrium,
    pub(crate) servantsquarters1: Atrium,
    pub(crate) gaolyard: Atrium,
    pub(crate) staff_room: Atrium,
    pub(crate) gaolhall: Atrium,
    pub(crate) sacredspring: Atrium,
    pub(crate) winecellar: Atrium,
    pub(crate) hallofmirrors: Atrium,
    pub(crate) gaolkitchen: Atrium,
    pub(crate) stairtenebrous: Atrium,
    pub(crate) earthcellar: Atrium,
    pub(crate) cell_adept: Atrium,
    #[serde(rename = "earlbrian'sfield")]
    pub(crate) earlbrian_sfield: Atrium,
    pub(crate) gullcolony: Atrium,
    pub(crate) hermitcell: Atrium,
    pub(crate) gullscrytower1: Atrium,
    #[serde(rename = "smuggler'sden")]
    pub(crate) smuggler_sden: Atrium,
    pub(crate) ourladybeneath: Atrium,
    pub(crate) gaolcellar: Atrium,
    pub(crate) bells: Atrium,
    pub(crate) spire: Atrium,
    pub(crate) gullscrytower2: Atrium,
    pub(crate) loadingdock: Atrium,
    pub(crate) gullscrytower3: Atrium,
    pub(crate) upperpumproom: Atrium,
    pub(crate) oubliette: Atrium,
    pub(crate) boathouse: Atrium,
    pub(crate) rowenarium: Atrium,
    pub(crate) whisperingstair: Atrium,
    pub(crate) hiddenstair: Atrium,
    pub(crate) crucibletower1: Atrium,
    pub(crate) salon: Atrium,
    pub(crate) run: Atrium,
    pub(crate) crucibletower2: Atrium,
    pub(crate) lowerpumproom: Atrium,
    pub(crate) seacaves: Atrium,
    pub(crate) hallofvoices: Atrium,
    pub(crate) crucibletower3: Atrium,
    pub(crate) silvervault: Atrium,
    pub(crate) columbiccrypt: Atrium,
    pub(crate) catacombs: Atrium,
    pub(crate) chapelcalicite: Atrium,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Atrium {
    #[serde(rename = "$type")]
    pub(crate) atrium_type: AtriumType,
    pub(crate) concern: String,
    pub(crate) effect: String,
    pub(crate) parameter: String,
    pub(crate) get_full_fx: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AtriumType {
    #[serde(rename = "SecretHistories.Commands.EnviroFxCommand, SecretHistories.Main")]
    SecretHistoriesCommandsEnviroFxCommandSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    // elements/incidents_weather
    Clouds,
    Earthquake,
    Fog,
    Gale,
    Hail,

    #[serde(rename(serialize = "Nume-Brume", deserialize = "nume_brume"))]
    NumeBrume,
    Rain,
    Snow,
    Storm,
    Sunny,

    // elements/celestial
    Afternoon,
    Autumn,
    #[serde(rename(serialize = "Autumn [Paused]", deserialize = "autumn_paused"))]
    AutumnPaused,
    Dawn,
    Daybreak,
    Dusk,
    Midday,
    Morning,
    Night,
    Numa,
    #[serde(rename(serialize = "Numa Next", deserialize = "numa_next"))]
    NumaNext,
    #[serde(rename(serialize = "Numa Not Yet", deserialize = "numa_not_yet"))]
    NumaNotYet,
    Spring,
    #[serde(rename(serialize = "Spring [Paused]", deserialize = "spring_paused"))]
    SpringPaused,
    Summer,
    #[serde(rename(serialize = "Summer [Paused]", deserialize = "summer_paused"))]
    SummerPaused,
    #[serde(rename(serialize = "Towards Numa", deserialize = "towards_numa"))]
    TowardsNuma,
    Winter,
    #[serde(rename(serialize = "Winter [Paused]", deserialize = "winter_paused"))]
    WinterPaused
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootPopulationCommand {
    #[serde(rename = "$type")]
    pub(crate) root_population_command_type: String,
    pub(crate) mutations: RootPopulationCommandMutations,
    pub(crate) spheres: Vec<RootPopulationCommandSphere>,
    pub(crate) dealers_table: DealersTable,
    pub(crate) tokens_at_arbitrary_paths: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealersTable {
    #[serde(rename = "$type")]
    pub(crate) dealers_table_type: DealersTableType,
    pub(crate) identifier: String,
    pub(crate) spheres: Vec<DealersTableSphere>,
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
    pub(crate) sphere_type: SphereType,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: PurpleGoverningSphereSpec,
    pub(crate) tokens: Vec<PurpleToken>,
    pub(crate) shrouded: bool,
    pub(crate) persistent_sphere_data: Option<serde_json::Value>,
    pub(crate) illuminations: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: GoverningSphereSpecType,
    pub(crate) label: String,
    pub(crate) action_id: String,
    pub(crate) description: String,
    pub(crate) available_from_house: String,
    pub(crate) essential: CurrentItineraries,
    pub(crate) required: CurrentItineraries,
    pub(crate) forbidden: CurrentItineraries,
    pub(crate) if_aspects_present: CurrentItineraries,
    pub(crate) greedy: bool,
    pub(crate) angels: Vec<Option<serde_json::Value>>,
    pub(crate) from_path: Path,
    pub(crate) en_route_sphere_path: Path,
    pub(crate) windows_sphere_path: Path,
    pub(crate) sphere_type: String,
    pub(crate) allow_any_token: bool,
    pub(crate) id: String,
    pub(crate) lever: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Path {
    #[serde(rename = "$type")]
    pub(crate) path_type: EnRouteSpherePathType,
    pub(crate) filter: Option<serde_json::Value>,
    pub(crate) path: String,
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
    pub(crate) token_type: TokenType,
    pub(crate) location: Location,
    pub(crate) home_location: Option<serde_json::Value>,
    pub(crate) payload: PurplePayload,
    pub(crate) placement_already_chronicled: bool,
    pub(crate) defunct: bool,
    pub(crate) current_state: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    #[serde(rename = "$type")]
    pub(crate) location_type: LocationType,
    pub(crate) local_position: Box<LocalPosition>,
    pub(crate) at_sphere_path: Path,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalPosition {
    #[serde(rename = "$type")]
    pub(crate) local_position_type: LocalPositionType,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) magnitude: f64,
    pub(crate) sqr_magnitude: f64,
    pub(crate) normalized: Option<Box<LocalPosition>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalPositionType {
    #[serde(rename = "UnityEngine.Vector3, UnityEngine.CoreModule")]
    UnityEngineVector3UnityEngineCoreModule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "SecretHistories.UI.TokenLocation, SecretHistories.Main")]
    SecretHistoriesUiTokenLocationSecretHistoriesMain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurplePayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: PayloadType,
    pub(crate) id: String,
    pub(crate) entity_id: String,
    pub(crate) quantity: i64,
    pub(crate) mutations: CurrentItineraries,
    pub(crate) illuminations: CurrentItineraries,
    pub(crate) defunct: bool,
    pub(crate) is_shrouded: bool,
    pub(crate) lifetime_remaining: f64,
    pub(crate) dominions: Vec<Option<serde_json::Value>>,
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
    pub(crate) mutations_type: String,
    pub(crate) ii: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootPopulationCommandSphere {
    #[serde(rename = "$type")]
    pub(crate) sphere_type: SphereType,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: FluffyGoverningSphereSpec,
    pub(crate) tokens: Vec<FluffyToken>,
    pub(crate) shrouded: bool,
    pub(crate) persistent_sphere_data: Option<PersistentSphereData>,
    pub(crate) illuminations: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: GoverningSphereSpecType,
    pub(crate) label: String,
    pub(crate) action_id: String,
    pub(crate) description: String,
    pub(crate) available_from_house: String,
    pub(crate) essential: CurrentItineraries,
    pub(crate) required: PurpleRequired,
    pub(crate) forbidden: PurpleForbidden,
    pub(crate) if_aspects_present: CurrentItineraries,
    pub(crate) greedy: bool,
    pub(crate) angels: Vec<Option<serde_json::Value>>,
    pub(crate) from_path: Path,
    pub(crate) en_route_sphere_path: Path,
    pub(crate) windows_sphere_path: Path,
    pub(crate) sphere_type: String,
    pub(crate) allow_any_token: bool,
    pub(crate) id: String,
    pub(crate) lever: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "Clouds mask the sky...")]
    CloudsMaskTheSky,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "[Keep things here until you tidy them away.]")]
    KeepThingsHereUntilYouTidyThemAway,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleForbidden {
    #[serde(rename = "$type")]
    pub(crate) forbidden_type: ForbiddenType,
    pub(crate) memory: Option<i64>,
    pub(crate) ability: Option<i64>,
    pub(crate) skill: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForbiddenType {
    #[serde(rename = "SecretHistories.Core.AspectsDictionary, SecretHistories.Main")]
    SecretHistoriesCoreAspectsDictionarySecretHistoriesMain,
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurpleRequired {
    #[serde(rename = "$type")]
    pub(crate) required_type: ForbiddenType,
    pub(crate) ability: Option<i64>,
    pub(crate) skill: Option<i64>,
    pub(crate) memory: Option<i64>,
    pub(crate) lesson: Option<i64>,
    pub(crate) circumstance: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PersistentSphereData {
    #[serde(rename = "$type")]
    pub(crate) persistent_sphere_data_type: String,
    pub(crate) default_position: f64,
    pub(crate) current_position: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyToken {
    #[serde(rename = "$type")]
    pub(crate) token_type: TokenType,
    pub(crate) location: Location,
    pub(crate) home_location: Option<Location>,
    pub(crate) payload: FluffyPayload,
    pub(crate) placement_already_chronicled: bool,
    pub(crate) defunct: bool,
    pub(crate) current_state: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyPayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: PayloadType,
    pub(crate) id: String,
    pub(crate) dominions: Vec<PurpleDominion>,
    pub(crate) mutations: FluffyMutations,
    pub(crate) edens_enacted: Option<Vec<EdensEnacted>>,
    pub(crate) is_sealed: Option<bool>,
    pub(crate) is_shrouded: Option<bool>,
    pub(crate) quantity: i64,
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
    pub(crate) dominion_type: DealersTableType,
    pub(crate) identifier: Identifier,
    pub(crate) spheres: Vec<PurpleSphere>,
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
    pub(crate) sphere_type: SphereType,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: TentacledGoverningSphereSpec,
    pub(crate) tokens: Vec<TentacledToken>,
    pub(crate) shrouded: bool,
    pub(crate) persistent_sphere_data: Option<serde_json::Value>,
    pub(crate) illuminations: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: GoverningSphereSpecType,
    pub(crate) label: String,
    pub(crate) action_id: String,
    pub(crate) description: String,
    pub(crate) available_from_house: AvailableFromHouse,
    pub(crate) essential: PurpleEssential,
    pub(crate) required: FluffyRequired,
    pub(crate) forbidden: FluffyForbidden,
    pub(crate) if_aspects_present: CurrentItineraries,
    pub(crate) greedy: bool,
    pub(crate) angels: Vec<Option<serde_json::Value>>,
    pub(crate) from_path: Path,
    pub(crate) en_route_sphere_path: Path,
    pub(crate) windows_sphere_path: Path,
    pub(crate) sphere_type: String,
    pub(crate) allow_any_token: bool,
    pub(crate) id: String,
    pub(crate) lever: String,
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
pub struct PurpleEssential {
    #[serde(rename = "$type")]
    pub(crate) essential_type: ForbiddenType,
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
    pub(crate) forbidden_type: ForbiddenType,
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
    pub(crate) required_type: ForbiddenType,
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
    pub(crate) token_type: TokenType,
    pub(crate) location: Location,
    pub(crate) home_location: Option<Location>,
    pub(crate) payload: TentacledPayload,
    pub(crate) placement_already_chronicled: bool,
    pub(crate) defunct: bool,
    pub(crate) current_state: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledPayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: PayloadType,
    pub(crate) last_situation_created: Option<serde_json::Value>,
    pub(crate) id: String,
    pub(crate) verb_id: Option<String>,
    pub(crate) output_path: Option<serde_json::Value>,
    pub(crate) current_recipe_id: Option<String>,
    pub(crate) fallback_recipe_id: Option<String>,
    pub(crate) quantity: i64,
    pub(crate) state_identifier: Option<i64>,
    pub(crate) time_remaining: Option<f64>,
    pub(crate) mutations: PurpleMutations,
    pub(crate) is_open: Option<bool>,
    pub(crate) dominions: Vec<FluffyDominion>,
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
    #[serde(rename = "NullRecipe")]
    NullRecipe,
    #[serde(rename = "salon.hint.raw")]
    SalonHintRaw,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyDominion {
    #[serde(rename = "$type")]
    pub(crate) dominion_type: DealersTableType,
    pub(crate) identifier: Identifier,
    pub(crate) spheres: Vec<FluffySphere>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffySphere {
    #[serde(rename = "$type")]
    pub(crate) sphere_type: SphereType,
    pub(crate) owner_sphere_identifier: Option<serde_json::Value>,
    pub(crate) governing_sphere_spec: StickyGoverningSphereSpec,
    pub(crate) tokens: Vec<StickyToken>,
    pub(crate) shrouded: bool,
    pub(crate) persistent_sphere_data: Option<serde_json::Value>,
    pub(crate) illuminations: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyGoverningSphereSpec {
    #[serde(rename = "$type")]
    pub(crate) governing_sphere_spec_type: GoverningSphereSpecType,
    pub(crate) label: String,
    pub(crate) action_id: String,
    pub(crate) description: String,
    pub(crate) available_from_house: String,
    pub(crate) essential: FluffyEssential,
    pub(crate) required: TentacledRequired,
    pub(crate) forbidden: TentacledForbidden,
    pub(crate) if_aspects_present: CurrentItineraries,
    pub(crate) greedy: bool,
    pub(crate) angels: Vec<Option<serde_json::Value>>,
    pub(crate) from_path: Path,
    pub(crate) en_route_sphere_path: Path,
    pub(crate) windows_sphere_path: Path,
    pub(crate) sphere_type: String,
    pub(crate) allow_any_token: bool,
    pub(crate) id: String,
    pub(crate) lever: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyEssential {
    #[serde(rename = "$type")]
    pub(crate) essential_type: ForbiddenType,
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
    pub(crate) forbidden_type: ForbiddenType,
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
    pub(crate) required_type: ForbiddenType,
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
    pub(crate) remains: Option<i64>,
    pub(crate) material: Option<i64>,
    pub(crate) carcass: Option<i64>,
    pub(crate) film: Option<i64>,
    pub(crate) moth: Option<i64>,
    pub(crate) flower: Option<i64>,
    pub(crate) liquid: Option<i64>,
    pub(crate) fuel: Option<i64>,
    #[serde(rename = "interest.moth")]
    pub(crate) interest_moth: Option<i64>,
    pub(crate) root: Option<i64>,
    pub(crate) leaf: Option<i64>,
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
    pub(crate) token_type: TokenType,
    pub(crate) location: Location,
    pub(crate) home_location: Option<serde_json::Value>,
    pub(crate) payload: StickyPayload,
    pub(crate) placement_already_chronicled: bool,
    pub(crate) defunct: bool,
    pub(crate) current_state: CurrentItineraries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyPayload {
    #[serde(rename = "$type")]
    pub(crate) payload_type: PayloadType,
    pub(crate) id: String,
    pub(crate) entity_id: String,
    pub(crate) quantity: i64,
    pub(crate) mutations: PurpleMutations,
    pub(crate) illuminations: Illuminations,
    pub(crate) defunct: bool,
    pub(crate) is_shrouded: bool,
    pub(crate) lifetime_remaining: f64,
    pub(crate) dominions: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntityId {
    #[serde(rename = "tlg.note")]
    TlgNote,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Illuminations {
    #[serde(rename = "$type")]
    pub(crate) illuminations_type: String,
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
    pub(crate) mutations_type: String,
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
    #[serde(rename = "mastery.nectar")]
    pub(crate) mastery_nectar: Option<i64>,
    #[serde(rename = "mastery.sky")]
    pub(crate) mastery_sky: Option<i64>,
    #[serde(rename = "mastery.lantern")]
    pub(crate) mastery_lantern: Option<i64>,
    #[serde(rename = "mastery.scale")]
    pub(crate) mastery_scale: Option<i64>,
    #[serde(rename = "tx.fraser")]
    pub(crate) tx_fraser: Option<i64>,
    #[serde(rename = "tx.aladim")]
    pub(crate) tx_aladim: Option<i64>,
    #[serde(rename = "tx.azita")]
    pub(crate) tx_azita: Option<i64>,
    #[serde(rename = "tx.chaima")]
    pub(crate) tx_chaima: Option<i64>,
    #[serde(rename = "mystery.grail")]
    pub(crate) mystery_grail: Option<i64>,
    pub(crate) soph: Option<i64>,
    #[serde(rename = "mystery.winter")]
    pub(crate) mystery_winter: Option<i64>,
    pub(crate) knock: Option<i64>,
    #[serde(rename = "mastery.winter")]
    pub(crate) mastery_winter: Option<i64>,
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
    #[serde(rename = "address.aladim")]
    pub(crate) address_aladim: Option<i64>,
    #[serde(rename = "contamination.curse.fifth.eye")]
    pub(crate) contamination_curse_fifth_eye: Option<i64>,
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
    pub(crate) last_run_recipe_type: LastRunRecipeType,
    pub(crate) priority: i64,
    pub(crate) action_id: ActionId,
    pub(crate) blocks: bool,
    pub(crate) reqs: Reqs,
    pub(crate) extant_reqs: CurrentItineraries,
    pub(crate) greq: CurrentItineraries,
    pub(crate) ngreq: CurrentItineraries,
    #[serde(rename = "FXReqs")]
    pub(crate) fx_reqs: CurrentItineraries,
    pub(crate) seeking: CurrentItineraries,
    pub(crate) effects: Effects,
    pub(crate) x_pans: CurrentItineraries,
    #[serde(rename = "FX")]
    pub(crate) fx: CurrentItineraries,
    pub(crate) aspects: Aspects,
    pub(crate) mutations: Vec<Option<serde_json::Value>>,
    pub(crate) purge: CurrentItineraries,
    pub(crate) halt_verb: CurrentItineraries,
    pub(crate) delete_verb: CurrentItineraries,
    pub(crate) achievements: Vec<Option<serde_json::Value>>,
    pub(crate) signal_important_loop: bool,
    pub(crate) audio_one_shot: Option<serde_json::Value>,
    pub(crate) signal_ending_flavour: i64,
    pub(crate) craftable: bool,
    pub(crate) notable: bool,
    pub(crate) hint_only: bool,
    pub(crate) ambittable: bool,
    pub(crate) warmup: i64,
    pub(crate) inherits: String,
    pub(crate) preface: String,
    pub(crate) start_label: String,
    pub(crate) label: String,
    pub(crate) start_description: String,
    pub(crate) desc: String,
    pub(crate) comments: String,
    pub(crate) deck_effects: CurrentItineraries,
    pub(crate) alt: Vec<Option<serde_json::Value>>,
    pub(crate) lalt: Vec<Option<serde_json::Value>>,
    pub(crate) inductions: Vec<Option<serde_json::Value>>,
    pub(crate) linked: Vec<Option<serde_json::Value>>,
    pub(crate) ending: String,
    pub(crate) icon: String,
    pub(crate) burn_image: Option<serde_json::Value>,
    pub(crate) run: String,
    pub(crate) pre_slots: Vec<Option<serde_json::Value>>,
    pub(crate) slots: Vec<Option<serde_json::Value>>,
    pub(crate) internal_deck: InternalDeck,
    pub(crate) id: Option<String>,
    pub(crate) lever: String,
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
    pub(crate) aspects_type: ForbiddenType,
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
    pub(crate) effects_type: String,
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
    pub(crate) internal_deck_type: InternalDeckType,
    pub(crate) default_card: String,
    pub(crate) reset_on_exhaustion: bool,
    pub(crate) label: String,
    pub(crate) desc: String,
    pub(crate) cover: Cover,
    pub(crate) comments: String,
    pub(crate) is_hidden: bool,
    pub(crate) draws: i64,
    pub(crate) spec: Vec<Option<serde_json::Value>>,
    pub(crate) draw_messages: CurrentItineraries,
    pub(crate) default_draw_messages: CurrentItineraries,
    pub(crate) id: Option<InternalDeckId>,
    pub(crate) lever: String,
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
    pub(crate) reqs_type: String,
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
    pub(crate) locked_in_recipe_type: LastRunRecipeType,
    pub(crate) priority: i64,
    pub(crate) action_id: ActionId,
    pub(crate) blocks: bool,
    pub(crate) reqs: CurrentItineraries,
    pub(crate) extant_reqs: CurrentItineraries,
    pub(crate) greq: CurrentItineraries,
    pub(crate) ngreq: CurrentItineraries,
    #[serde(rename = "FXReqs")]
    pub(crate) fx_reqs: CurrentItineraries,
    pub(crate) seeking: CurrentItineraries,
    pub(crate) effects: CurrentItineraries,
    pub(crate) x_pans: CurrentItineraries,
    #[serde(rename = "FX")]
    pub(crate) fx: CurrentItineraries,
    pub(crate) aspects: CurrentItineraries,
    pub(crate) mutations: Vec<Option<serde_json::Value>>,
    pub(crate) purge: CurrentItineraries,
    pub(crate) halt_verb: CurrentItineraries,
    pub(crate) delete_verb: CurrentItineraries,
    pub(crate) achievements: Vec<Option<serde_json::Value>>,
    pub(crate) signal_important_loop: bool,
    pub(crate) audio_one_shot: Option<serde_json::Value>,
    pub(crate) signal_ending_flavour: i64,
    pub(crate) craftable: bool,
    pub(crate) notable: bool,
    pub(crate) hint_only: bool,
    pub(crate) ambittable: bool,
    pub(crate) warmup: i64,
    pub(crate) inherits: String,
    pub(crate) preface: String,
    pub(crate) start_label: String,
    pub(crate) label: String,
    pub(crate) start_description: String,
    pub(crate) desc: String,
    pub(crate) comments: String,
    pub(crate) deck_effects: CurrentItineraries,
    pub(crate) alt: Vec<Option<serde_json::Value>>,
    pub(crate) lalt: Vec<Option<serde_json::Value>>,
    pub(crate) inductions: Vec<Option<serde_json::Value>>,
    pub(crate) linked: Vec<Option<serde_json::Value>>,
    pub(crate) ending: String,
    pub(crate) icon: ActionId,
    pub(crate) burn_image: Option<serde_json::Value>,
    pub(crate) run: String,
    pub(crate) pre_slots: Vec<Option<serde_json::Value>>,
    pub(crate) slots: Vec<Option<serde_json::Value>>,
    pub(crate) internal_deck: InternalDeck,
    pub(crate) id: String,
    pub(crate) lever: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluffyMutations {
    #[serde(rename = "$type")]
    pub(crate) mutations_type: String,
    #[serde(rename = "v.numa.winter.a")]
    pub(crate) v_numa_winter_a: Option<i64>,
    #[serde(rename = "v.numa.moon.b")]
    pub(crate) v_numa_moon_b: Option<i64>,
    #[serde(rename = "interest.moon")]
    pub(crate) interest_moon: Option<i64>,
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
    #[serde(rename = "v.rite.renewal")]
    pub(crate) v_rite_renewal: Option<i64>,
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
    #[serde(rename = "acted.fraser.hunt.changing.scale")]
    pub(crate) acted_fraser_hunt_changing_scale: Option<i64>,
    #[serde(rename = "acted.azita.hunt.changing.edge")]
    pub(crate) acted_azita_hunt_changing_edge: Option<i64>,
    #[serde(rename = "acted.arthur.hunt.changing.edge")]
    pub(crate) acted_arthur_hunt_changing_edge: Option<i64>,
    #[serde(rename = "acted.chaima.hunt.changing.edge")]
    pub(crate) acted_chaima_hunt_changing_edge: Option<i64>,
    #[serde(rename = "acted.aladim.hunt.changing.scale")]
    pub(crate) acted_aladim_hunt_changing_scale: Option<i64>,
    #[serde(rename = "wisdom.committed")]
    pub(crate) wisdom_committed: Option<i64>,
    #[serde(rename = "w.birdsong")]
    pub(crate) w_birdsong: Option<i64>,
    pub(crate) moon: Option<i64>,
    pub(crate) scale: Option<i64>,
    pub(crate) skill: Option<i64>,
    pub(crate) lantern: Option<i64>,
    pub(crate) sky: Option<i64>,
    pub(crate) forge: Option<i64>,
    pub(crate) edge: Option<i64>,
    #[serde(rename = "a.xhea")]
    pub(crate) a_xhea: Option<i64>,
    #[serde(rename = "w.nyctodromy")]
    pub(crate) w_nyctodromy: Option<i64>,
    pub(crate) rose: Option<i64>,
    #[serde(rename = "a.xfet")]
    pub(crate) a_xfet: Option<i64>,
    #[serde(rename = "a.xpho")]
    pub(crate) a_xpho: Option<i64>,
    #[serde(rename = "w.hushery")]
    pub(crate) w_hushery: Option<i64>,
    #[serde(rename = "w.illumination")]
    pub(crate) w_illumination: Option<i64>,
    #[serde(rename = "w.bosk")]
    pub(crate) w_bosk: Option<i64>,
    #[serde(rename = "a.xhausted")]
    pub(crate) a_xhausted: Option<i64>,
    pub(crate) heart: Option<i64>,
    pub(crate) nectar: Option<i64>,
    pub(crate) winter: Option<i64>,
    #[serde(rename = "w.ithastry")]
    pub(crate) w_ithastry: Option<i64>,
    pub(crate) grail: Option<i64>,
    #[serde(rename = "w.skolekosophy")]
    pub(crate) w_skolekosophy: Option<i64>,
    #[serde(rename = "w.horomachistry")]
    pub(crate) w_horomachistry: Option<i64>,
    pub(crate) knock: Option<i64>,
    #[serde(rename = "a.xere")]
    pub(crate) a_xere: Option<i64>,
    #[serde(rename = "a.xsha")]
    pub(crate) a_xsha: Option<i64>,
    #[serde(rename = "a.xmet")]
    pub(crate) a_xmet: Option<i64>,
    #[serde(rename = "a.xtri")]
    pub(crate) a_xtri: Option<i64>,
    #[serde(rename = "w.preservation")]
    pub(crate) w_preservation: Option<i64>,
    #[serde(rename = "a.xcho")]
    pub(crate) a_xcho: Option<i64>,
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
}
