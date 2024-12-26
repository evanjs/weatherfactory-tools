use crate::model::{FindById, GameElementDetails, Identifiable};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipes {
    #[serde(rename = "recipes")]
    pub(crate) elements: Vec<Element>,
}

impl IntoIterator for Recipes {
    type Item = Element;
    type IntoIter = std::vec::IntoIter<Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum BoolOrString {
    Bool(bool),
    String(String),
}

impl From<BoolOrString> for bool {
    fn from(val: BoolOrString) -> Self {
        match val {
            BoolOrString::Bool(b) => b,
            BoolOrString::String(s) => s.parse().unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub(crate) id: String,
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) actionid: Option<Actionid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reqs: Option<Reqs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) effects: Option<HashMap<String, ReqValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) aspects: Option<Aspects>,
    #[serde(rename = "StartDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_description: Option<String>,
    #[serde(rename = "Desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) warmup: Option<ReqValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) craftable: Option<BoolOrString>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ReqValue {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reqs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) restorative: Option<i64>,
    #[serde(rename = "malady.cure.forge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_forge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) forge: Option<ReqValue>,
    #[serde(rename = "malady.cure.grail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_grail: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grail: Option<ReqValue>,
    #[serde(rename = "malady.cure.heart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_heart: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) heart: Option<ReqValue>,
    #[serde(rename = "malady.cure.lantern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_lantern: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) lantern: Option<ReqValue>,
    #[serde(rename = "malady.cure.moth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_moth: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) moth: Option<ReqValue>,
    #[serde(rename = "malady.cure.nectar")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_nectar: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) nectar: Option<ReqValue>,
    #[serde(rename = "malady.cure.sky")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_sky: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sky: Option<ReqValue>,
    #[serde(rename = "malady.cure.winter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady_cure_winter: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) winter: Option<ReqValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ability: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fatigued: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) skill: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) malady: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) memory: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thing: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) visitor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) weather: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actionid {
    #[serde(rename = "ch")]
    Ch,
    #[serde(rename = "ch.setup")]
    ChSetup,
    #[serde(rename = "*.consider")]
    GlobDotConsider,
    #[serde(rename = "*")]
    Glob,
    #[serde(rename = "*consider")]
    GlobConsider,
    #[serde(rename = "block.numa.initial")]
    BlockNumaInitial,
    #[serde(rename = "collection.busts")]
    CollectionBusts,
    #[serde(rename = "collection.test")]
    CollectionTest,
    #[serde(rename = "consider.setup")]
    ConsiderSetup,
    #[serde(rename = "consider")]
    Consider,
    #[serde(rename = "desk")]
    Desk,
    #[serde(rename = "desk*")]
    DeskGlob,
    #[serde(rename = "draw.numa")]
    DrawNuma,
    #[serde(rename = "garden.*")]
    GardenDotGlob,
    #[serde(rename = "garden.beehive")]
    GardenBeehive,
    #[serde(rename = "garden.beehive*")]
    GardenBeehiveGlob,
    #[serde(rename = "garden.beehive.liselotte")]
    GardenBeehiveLiselotte,
    #[serde(rename = "garden.plot.*")]
    GardenPlotDotGlob,
    #[serde(rename = "garden.plot.a.*")]
    GardenPlotADotGlob,
    #[serde(rename = "garden.plot.b.*")]
    GardenPlotBDotGlob,
    #[serde(rename = "garden.plot.c.*")]
    GardenPlotCDotGlob,
    #[serde(rename = "garden.plot.d.*")]
    GardenPlotDDotGlob,
    #[serde(rename = "garden.plot.g.*")]
    GardenPlotGDotGlob,
    #[serde(rename = "garden.plot.h.*")]
    GardenPlotHDotGlob,
    #[serde(rename = "garden.plot.p*")]
    GardenPlotPGlob,
    #[serde(rename = "garden.plot.p.*")]
    GardenPlotPDotGlob,
    #[serde(rename = "garden.plot.v.*")]
    GardenPlotVDotGlob,
    #[serde(rename = "garden.plot.x.*")]
    GardenPlotXDotGlob,
    #[serde(rename = "incident.advance")]
    IncidentAdvance,
    #[serde(rename = "incident.base")]
    IncidentBase,
    #[serde(rename = "incident.box")]
    IncidentBox,
    #[serde(rename = "incident.consultation")]
    IncidentConsultation,
    #[serde(rename = "incident.curse")]
    IncidentCurse,
    #[serde(rename = "incident.draw.consultation")]
    IncidentDrawConsultation,
    #[serde(rename = "incident.expulsion.finisher")]
    IncidentExpulsionFinisher,
    #[serde(rename = "incident.heist")]
    IncidentHeist,
    #[serde(rename = "incident.hunt.changing")]
    IncidentHuntChanging,
    #[serde(rename = "incident.intrusion")]
    IncidentIntrusion,
    #[serde(rename = "incident.lost.find")]
    IncidentLostFind,
    #[serde(rename = "incident.mark.finished")]
    IncidentMarkFinished,
    #[serde(rename = "incident.mob")]
    IncidentMob,
    #[serde(rename = "incident.mystical")]
    IncidentMystical,
    #[serde(rename = "incident.new.numa")]
    IncidentNewNuma,
    #[serde(rename = "incident.new")]
    IncidentNew,
    #[serde(rename = "incident.numa.grail.a")]
    IncidentNumaGrailA,
    #[serde(rename = "incident.numa.grail.b")]
    IncidentNumaGrailB,
    #[serde(rename = "incident.numa.moon.a")]
    IncidentNumaMoonA,
    #[serde(rename = "incident.numa.moon.b")]
    IncidentNumaMoonB,
    #[serde(rename = "incident.numa.winter.a")]
    IncidentNumaWinterA,
    #[serde(rename = "incident.numa.wood")]
    IncidentNumaWood,
    #[serde(rename = "incident.observe.scaly")]
    IncidentObserveScaly,
    #[serde(rename = "incident.omen.dawn")]
    IncidentOmenDawn,
    #[serde(rename = "incident.omen.horizon")]
    IncidentOmenHorizon,
    #[serde(rename = "incident.opera.apollo")]
    IncidentOperaApollo,
    #[serde(rename = "incident.opera.wings")]
    IncidentOperaWings,
    #[serde(rename = "incident.pan")]
    IncidentPan,
    #[serde(rename = "incident.project.ingenious")]
    IncidentProjectIngenious,
    #[serde(rename = "incident.project.solar")]
    IncidentProjectSolar,
    #[serde(rename = "incident.retire.numa")]
    IncidentRetireNuma,
    #[serde(rename = "incident.retire")]
    IncidentRetire,
    #[serde(rename = "incident.revolution")]
    IncidentRevolution,
    #[serde(rename = "incident.rising")]
    IncidentRising,
    #[serde(rename = "incident.rite.awakening")]
    IncidentRiteAwakening,
    #[serde(rename = "incident.rite.renewal")]
    IncidentRiteRenewal,
    #[serde(rename = "incident.setup.numa")]
    IncidentSetupNuma,
    #[serde(rename = "incident.setup")]
    IncidentSetup,
    #[serde(rename = "incident.stalk")]
    IncidentStalk,
    #[serde(rename = "incident.wound")]
    IncidentWound,
    #[serde(rename = "leiter")]
    Leiter,
    #[serde(rename = "library.*")]
    LibraryDotGlob,
    #[serde(rename = "library.altar.chancel")]
    LibraryAltarChancel,
    #[serde(rename = "library.altar.chancel*")]
    LibraryAltarChancelGlob,
    #[serde(rename = "library.altar.lady")]
    LibraryAltarLady,
    #[serde(rename = "library.altar.tentreto")]
    LibraryAltarTentreto,
    #[serde(rename = "library.altar.tentreto*")]
    LibraryAltarTentretoGlob,
    #[serde(rename = "library.bed.*")]
    LibraryBedDotGlob,
    #[serde(rename = "library.bed.guest.*")]
    LibraryBedGuestDotGlob,
    #[serde(rename = "library.bed.rest.*")]
    LibraryBedRestDotGlob,
    #[serde(rename = "library.bed.rest*")]
    LibraryBedRestGlob,
    #[serde(rename = "library.chrysalis")]
    LibraryChrysalis,
    #[serde(rename = "library.desk.*")]
    LibraryDeskDotGlob,
    #[serde(rename = "library.fireplace.*")]
    LibraryFireplaceDotGlob,
    #[serde(rename = "library.foundry")]
    LibraryFoundry,
    #[serde(rename = "library.kitchen.*")]
    LibraryKitchenDotGlob,
    #[serde(rename = "library.oubliette")]
    LibraryOubliette,
    #[serde(rename = "library.phonograph.*")]
    LibraryPhonographDotGlob,
    #[serde(rename = "library.projector.*")]
    LibraryProjectorDotGlob,
    #[serde(rename = "library.table.necropsy")]
    LibraryTableNecropsy,
    #[serde(rename = "library.rowenarium")]
    LibraryRowenarium,
    #[serde(rename = "library.rowenarium*")]
    LibraryRowenariumGlob,
    #[serde(rename = "library.sarcophagus.columbic")]
    LibrarySarcophagusColumbic,
    #[serde(rename = "library.well")]
    LibraryWell,
    #[serde(rename = "library.workbench.*")]
    LibraryWorkbenchDotGlob,
    #[serde(rename = "lighthouse")]
    Lighthouse,
    #[serde(rename = "lighthouse.convene")]
    LighthouseConvene,
    #[serde(rename = "neiter")]
    Neiter,
    #[serde(rename = "numa.incident.base")]
    NumaIncidentBase,
    #[serde(rename = "numa.killes")]
    NumaKilles,
    #[serde(rename = "numa.other.visitors.arrive")]
    NumaOtherVisitorsArrive,
    #[serde(rename = "numa.possibility")]
    NumaPossibility,
    #[serde(rename = "numa.rectory")]
    NumaRectory,
    #[serde(rename = "numa.smithy")]
    NumaSmithy,
    #[serde(rename = "nx")]
    Nx,
    #[serde(rename = "nx.resurrect")]
    NxResurrect,

    #[serde(rename = "oriflammes.draw")]
    OriflammesDraw,
    #[serde(rename = "oriflammes.offer")]
    OriflammesOffer,
    #[serde(rename = "oriflammes.pass")]
    OriflammesPass,
    #[serde(rename = "oriflammes.setup")]
    OriflammesSetup,

    #[serde(rename = "postoffice.delivery")]
    PostOfficeDelivery,
    #[serde(rename = "pumps.lower")]
    PumpsLower,
    #[serde(rename = "pumps.upper")]
    PumpsUpper,

    #[serde(rename = "rhonwen.correspondence.trysend")]
    RhonwenCorrespondenceTrySend,
    #[serde(rename = "rhonwen.correspondence")]
    RhonwenCorrespondence,
    #[serde(rename = "rhonwen.reminder.send")]
    RhonwenReminderSend,
    #[serde(rename = "rhonwen.reminder.trysend")]
    RhonwenReminderTrySend,
    #[serde(rename = "rhonwen.setup")]
    RhonwenSetup,
    #[serde(rename = "rhonwen.stipend")]
    RhonwenStipend,

    #[serde(rename = "salon.*")]
    SalonDotGlob,
    #[serde(rename = "station.serving.*")]
    StationServingDotGlob,
    #[serde(rename = "season")]
    Season,
    #[serde(rename = "talk.setup")]
    TalkSetup,
    #[serde(rename = "talk")]
    Talk,
    #[serde(rename = "terrain.unlock")]
    TerrainUnlock,
    #[serde(rename = "time")]
    Time,

    #[serde(rename = "trn")]
    Trn,
    #[serde(rename = "trn.setup")]
    TrnSetup,

    #[serde(rename = "unknown")]
    Unknown,

    #[serde(rename = "village.*")]
    VillageDotGlob,
    #[serde(rename = "village.invite.denzil.block.hint")]
    VillageInviteDenzilBlockHint,
    #[serde(rename = "village.invite.denzil.stub")]
    VillageInviteDenzilStub,
    #[serde(rename = "village.invite.denzil")]
    VillageInviteDenzil,
    #[serde(rename = "village.invite.timothy.block.hint")]
    VillageInviteTimothyBlockHint,
    #[serde(rename = "village.invite.timothy.stub")]
    VillageInviteTimothyStub,
    #[serde(rename = "village.invite.timothy")]
    VillageInviteTimothy,
    #[serde(rename = "village.killes")]
    VillageKilles,
    #[serde(rename = "village.killes.block.hint")]
    VillageKillesBlockHint,
    #[serde(rename = "village.killes*")]
    VillageKillesGlob,
    #[serde(rename = "village.killes.open")]
    VillageKillesOpen,
    #[serde(rename = "village.killes.open.friend")]
    VillageKillesOpenFriend,
    #[serde(rename = "village.killes.open*")]
    VillageKillesOpenGlob,
    #[serde(rename = "village.killes.closed")]
    VillageKillesClosed,
    #[serde(rename = "village.killes.stub")]
    VillageKillesStub,
    #[serde(rename = "village.postoffice.open")]
    VillagePostOfficeOpen,
    #[serde(rename = "village.postoffice.closed")]
    VillagePostOfficeClosed,
    #[serde(rename = "village.rectory")]
    VillageRectory,
    #[serde(rename = "village.rectory*")]
    VillageRectoryGlob,
    #[serde(rename = "village.rectory.open")]
    VillageRectoryOpen,
    #[serde(rename = "village.rectory.open.friend")]
    VillageRectoryOpenFriend,
    #[serde(rename = "village.rectory.open*")]
    VillageRectoryOpenGlob,
    #[serde(rename = "village.rectory.closed")]
    VillageRectoryClosed,
    #[serde(rename = "village.smithy")]
    VillageSmithy,
    #[serde(rename = "village.smithy*")]
    VillageSmithyGlob,
    #[serde(rename = "village.smithy.open")]
    VillageSmithyOpen,
    #[serde(rename = "village.smithy.open.friend")]
    VillageSmithyOpenFriend,
    #[serde(rename = "village.smithy.open*")]
    VillageSmithyOpenGlob,
    #[serde(rename = "village.smithy.closed")]
    VillageSmithyClosed,
    #[serde(rename = "village.sweetbones.open")]
    VillageSweetbonesOpen,
    #[serde(rename = "village.sweetbones.open*")]
    VillageSweetbonesOpenGlob,
    #[serde(rename = "village.sweetbones.closed")]
    VillageSweetbonesClosed,

    #[serde(rename = "visitor.agdistis")]
    VisitorAgdistis,
    #[serde(rename = "visitor.aladim")]
    VisitorAladim,
    #[serde(rename = "visitor.arthur")]
    VisitorArthur,
    #[serde(rename = "visitor.arun")]
    VisitorArun,
    #[serde(rename = "visitor.azita")]
    VisitorAzita,
    #[serde(rename = "visitor.chaima")]
    VisitorChaima,
    #[serde(rename = "visitor.connie")]
    VisitorConnie,
    #[serde(rename = "visitor.coquille")]
    VisitorCoquille,
    #[serde(rename = "visitor.corso")]
    VisitorCorso,
    #[serde(rename = "visitor.dagmar")]
    VisitorDagmar,
    #[serde(rename = "visitor.douglas")]
    VisitorDouglas,
    #[serde(rename = "visitor.ehsan")]
    VisitorEhsan,
    #[serde(rename = "visitor.embarking.sendoffstage")]
    VisitorEmbarkingSendoffstage,
    #[serde(rename = "visitor.embarking")]
    VisitorEmbarking,
    #[serde(rename = "visitor.for.box")]
    VisitorForBox,
    #[serde(rename = "visitor.for.consultation.numa")]
    VisitorForConsultationNuma,
    #[serde(rename = "visitor.for.consultation")]
    VisitorForConsultation,
    #[serde(rename = "visitor.for.curse")]
    VisitorForCurse,
    #[serde(rename = "visitor.for.heist")]
    VisitorForHeist,
    #[serde(rename = "visitor.for.hunt.changing")]
    VisitorForHuntChanging,
    #[serde(rename = "visitor.for.intrusion")]
    VisitorForIntrusion,
    #[serde(rename = "visitor.for.lost.find")]
    VisitorForLostFind,
    #[serde(rename = "visitor.for.mob")]
    VisitorForMob,
    #[serde(rename = "visitor.for.mystical")]
    VisitorForMystical,
    #[serde(rename = "visitor.for.numa.grail.a")]
    VisitorForNumaGrailA,
    #[serde(rename = "visitor.for.numa.grail.b")]
    VisitorForNumaGrailB,
    #[serde(rename = "visitor.for.numa.moon.a")]
    VisitorForNumaMoonA,
    #[serde(rename = "visitor.for.numa.moon.b")]
    VisitorForNumaMoonB,
    #[serde(rename = "visitor.for.numa.winter.a")]
    VisitorForNumaWinterA,
    #[serde(rename = "visitor.for.numa.wood")]
    VisitorForNumaWood,
    #[serde(rename = "visitor.for.observe.scaly")]
    VisitorForObserveScaly,
    #[serde(rename = "visitor.for.omen.dawn")]
    VisitorForOmenDawn,
    #[serde(rename = "visitor.for.omen.horizon")]
    VisitorForOmenHorizon,
    #[serde(rename = "visitor.for.opera.apollo")]
    VisitorForOperaApollo,
    #[serde(rename = "visitor.for.opera.wings")]
    VisitorForOperaWings,
    #[serde(rename = "visitor.for.pan")]
    VisitorForPan,
    #[serde(rename = "visitor.for.project.ingenious")]
    VisitorForProjectIngenious,
    #[serde(rename = "visitor.for.project.solar")]
    VisitorForProjectSolar,
    #[serde(rename = "visitor.for.revolution")]
    VisitorForRevolution,
    #[serde(rename = "visitor.for.rising")]
    VisitorForRising,
    #[serde(rename = "visitor.for.rite.awakening")]
    VisitorForRiteAwakening,
    #[serde(rename = "visitor.for.rite.renewal")]
    VisitorForRiteRenewal,
    #[serde(rename = "visitor.for.stalk")]
    VisitorForStalk,
    #[serde(rename = "visitor.for.wound")]
    VisitorForWound,
    #[serde(rename = "visitor.fraser")]
    VisitorFraser,
    #[serde(rename = "visitor.hokobald")]
    VisitorHokobald,
    #[serde(rename = "visitor.morgen")]
    VisitorMorgen,
    #[serde(rename = "visitor.olympe")]
    VisitorOlympe,
    #[serde(rename = "visitorpatrol")]
    VisitorPatrol,
    #[serde(rename = "visitor.serena")]
    VisitorSerena,
    #[serde(rename = "visitor.stanislav")]
    VisitorStanislav,
    #[serde(rename = "visitor.villager")]
    VisitorVillager,
    #[serde(rename = "visitor.yvette")]
    VisitorYvette,
    #[serde(rename = "visitor.zachary")]
    VisitorZachary,

    #[serde(rename = "weather.enactor.clouds")]
    WeatherEnactorClouds,
    #[serde(rename = "weather.enactor.fog")]
    WeatherEnactorFog,
    #[serde(rename = "weather.enactor.gale")]
    WeatherEnactorGale,
    #[serde(rename = "weather.enactor.hail")]
    WeatherEnactorHail,
    #[serde(rename = "weather.enactor.numa")]
    WeatherEnactorNuma,
    #[serde(rename = "weather.enactor.rain")]
    WeatherEnactorRain,
    #[serde(rename = "weather.enactor.snow")]
    WeatherEnactorSnow,
    #[serde(rename = "weather.enactor.storm")]
    WeatherEnactorStorm,
    #[serde(rename = "weather.enactor.sunny")]
    WeatherEnactorSunny,
    #[serde(rename = "weather")]
    Weather,

    #[serde(rename = "wc.setup")]
    WcSetup,

    #[serde(rename = "world.beachcombing")]
    WorldBeachcombing,
    #[serde(rename = "world.corals.deeplight")]
    WorldCoralsDeeplight,
    #[serde(rename = "world.caves")]
    WorldCaves,
    #[serde(rename = "world.moor")]
    WorldMoor,
    #[serde(rename = "world.nests")]
    WorldNests,
    #[serde(rename = "world.sea*")]
    WorldSeaGlob,
    #[serde(rename = "world.sea.*")]
    WorldSeaDotGlob,
    #[serde(rename = "world.tree.gate")]
    WorldTreeGate,

    #[serde(rename = "wt.memorylocus")]
    WtMemorylocus,

    #[serde(rename = "wt.bir")]
    WtBirdsong,
    #[serde(rename = "wt.bir.*")]
    WtBirdsongDotGlob,
    #[serde(rename = "wt.bos")]
    WtBosk,
    #[serde(rename = "wt.bos.*")]
    WtBoskDotGlob,
    #[serde(rename = "wt.hor")]
    WtHoromachistry,
    #[serde(rename = "wt.hor.*")]
    WtHoromachistryDotGlob,
    #[serde(rename = "wt.hus")]
    WtHushery,
    #[serde(rename = "wt.hus.*")]
    WtHusheryDotGlob,
    #[serde(rename = "wt.ill")]
    WtIllumination,
    #[serde(rename = "wt.ill.*")]
    WtIlluminationDotGlob,
    #[serde(rename = "wt.ith")]
    WtIthastry,
    #[serde(rename = "wt.ith.*")]
    WtIthastryDotGlob,
    #[serde(rename = "wt.nyc")]
    WtNyctodromy,
    #[serde(rename = "wt.nyc.*")]
    WtNyctodromyDotGlob,
    #[serde(rename = "wt.pre")]
    WtPreservation,
    #[serde(rename = "wt.pre.*")]
    WtPreservationDotGlob,
    #[serde(rename = "wt.sko")]
    WtSkolekosophy,
    #[serde(rename = "wt.sko.*")]
    WtSkolekosophyDotGlob,
    #[serde(rename = "x")]
    X,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aspects {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fatiguing: Option<i64>,
}

impl From<Value> for Recipes {
    fn from(value: Value) -> Self {
        serde_json_path_to_error::from_value(value).unwrap()
    }
}

impl From<Recipes> for Value {
    fn from(val: Recipes) -> Self {
        serde_json::to_value(val).unwrap()
    }
}

impl Identifiable for Element {
    fn id(&self) -> &str {
        &self.id
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for Recipes {
    type Item = Element;

    type Collection = Vec<Element>;

    #[tracing::instrument(skip(self))]
    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }

    #[tracing::instrument(skip(self))]
    fn get_collection_mut(&mut self) -> &mut Self::Collection {
        self.elements.get_collection_mut()
    }
}

impl GameElementDetails for Element {
    fn get_label(&self) -> String {
        self.label.as_ref().unwrap_or(&"N/A".to_string()).clone()
    }
    fn get_desc(&self) -> String {
        self.clone().desc.unwrap_or_default().clone()
    }
}
