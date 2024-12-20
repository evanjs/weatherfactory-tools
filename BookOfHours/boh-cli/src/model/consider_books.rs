use crate::model::{FindById, GameCollectionType, GameElementDetails, Identifiable};
use crate::QueryType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsiderBooks {
    #[serde(rename = "recipes")]
    pub(crate) elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub(crate) id: String,
    pub(crate) actionid: Option<Actionid>,
    pub(crate) reqs: Option<Reqs>,
    pub(crate) startdescription: Option<String>,
    pub(crate) desc: Option<String>,
    pub(crate) warmup: Option<i64>,
    pub(crate) craftable: Option<bool>,
    pub(crate) label: Option<String>,
    pub(crate) effects: Option<Effects>,
    pub(crate) aspects: Option<HashMap<String, i64>>,
    pub(crate) extantreqs: Option<Extantreqs>,
    pub(crate) comments: Option<String>,
    pub(crate) mutations: Option<Vec<Mutation>>,
    pub(crate) inherits: Option<String>,
    pub(crate) deckeffects: Option<HashMap<String, i64>>,
    pub(crate) linked: Option<Vec<Linked>>,
    pub(crate) alt: Option<Vec<Alt>>,
    pub(crate) slots: Option<Vec<Slot>>,
    pub(crate) hintonly: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Actionid {
    #[serde(rename = "consider")]
    ActionidConsider,
    #[serde(rename = "*consider")]
    Consider,
    #[serde(rename = "*")]
    Empty,
    #[serde(rename = "library.phonograph.*")]
    LibraryPhonograph,
    #[serde(rename = "library.projector.*")]
    LibraryProjector,
    X,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alt {
    pub(crate) id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effects {
    pub(crate) uncatalogued: Option<i64>,
    #[serde(rename = "o.ability")]
    pub(crate) o_ability: Option<i64>,
    #[serde(rename = "o.weather")]
    pub(crate) o_weather: Option<i64>,
    #[serde(rename = "o.memory")]
    pub(crate) o_memory: Option<i64>,
    #[serde(rename = "o.comfort")]
    pub(crate) o_comfort: Option<i64>,
    #[serde(rename = "o.thing")]
    pub(crate) o_thing: Option<i64>,
    #[serde(rename = "o.wallart")]
    pub(crate) o_wallart: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extantreqs {
    #[serde(rename = "status.rhonwen.open")]
    pub(crate) status_rhonwen_open: Option<i64>,
    #[serde(rename = "time.night")]
    pub(crate) time_night: Option<i64>,
    pub(crate) skill: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Linked {
    pub(crate) id: Option<String>,
    pub(crate) chance: Option<i64>,
    pub(crate) additional: Option<bool>,
    pub(crate) expulsion: Option<Expulsion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expulsion {
    pub(crate) limit: Option<i64>,
    pub(crate) filter: Option<Filter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub(crate) zcho: Option<i64>,
    pub(crate) zere: Option<i64>,
    pub(crate) zfet: Option<i64>,
    pub(crate) zhea: Option<i64>,
    pub(crate) zpho: Option<i64>,
    pub(crate) zsha: Option<i64>,
    pub(crate) ztri: Option<i64>,
    pub(crate) zwis: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mutation {
    pub(crate) filter: Option<String>,
    pub(crate) mutate: Option<String>,
    pub(crate) level: Option<Edge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Edge {
    Integer(i64),
    String(String),
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reqs {
    pub(crate) readable: Option<i64>,
    pub(crate) soaked: Option<i64>,
    pub(crate) uncatalogued: Option<i64>,
    pub(crate) ability: Option<i64>,
    #[serde(rename = "ability.exposed.actinic")]
    pub(crate) ability_exposed_actinic: Option<i64>,
    #[serde(rename = "ability.exposed.bloodlines")]
    pub(crate) ability_exposed_bloodlines: Option<i64>,
    #[serde(rename = "ability.exposed.chionic")]
    pub(crate) ability_exposed_chionic: Option<i64>,
    #[serde(rename = "ability.exposed.keeperskin")]
    pub(crate) ability_exposed_keeperskin: Option<i64>,
    #[serde(rename = "ability.exposed.winkwell")]
    pub(crate) ability_exposed_winkwell: Option<i64>,
    #[serde(rename = "ability.exposed.witchworms")]
    pub(crate) ability_exposed_witchworms: Option<i64>,
    #[serde(rename = "uncatbook.dawn")]
    pub(crate) uncatbook_dawn: Option<i64>,
    #[serde(rename = "uncatbook.solar")]
    pub(crate) uncatbook_solar: Option<i64>,
    #[serde(rename = "uncatbook.baronial")]
    pub(crate) uncatbook_baronial: Option<i64>,
    #[serde(rename = "uncatbook.curia")]
    pub(crate) uncatbook_curia: Option<i64>,
    #[serde(rename = "uncatbook.nocturnal")]
    pub(crate) uncatbook_nocturnal: Option<i64>,
    #[serde(rename = "uncatbook.nocturnal.everyman")]
    pub(crate) uncatbook_nocturnal_everyman: Option<i64>,
    #[serde(rename = "uncatpile.dawn")]
    pub(crate) uncatpile_dawn: Option<i64>,
    pub(crate) count: Option<i64>,
    pub(crate) rose: Option<Edge>,
    #[serde(rename = "mystery.edge")]
    pub(crate) mystery_edge: Option<i64>,
    pub(crate) sky: Option<Edge>,
    pub(crate) knock: Option<Edge>,
    #[serde(rename = "mystery.winter")]
    pub(crate) mystery_winter: Option<i64>,
    #[serde(rename = "mystery.lantern")]
    pub(crate) mystery_lantern: Option<i64>,
    #[serde(rename = "mystery.grail")]
    pub(crate) mystery_grail: Option<i64>,
    #[serde(rename = "o.ability")]
    pub(crate) o_ability: Option<i64>,
    #[serde(rename = "o.weather")]
    pub(crate) o_weather: Option<i64>,
    #[serde(rename = "o.memory")]
    pub(crate) o_memory: Option<i64>,
    #[serde(rename = "o.comfort")]
    pub(crate) o_comfort: Option<i64>,
    #[serde(rename = "o.thing")]
    pub(crate) o_thing: Option<i64>,
    #[serde(rename = "o.wallart")]
    pub(crate) o_wallart: Option<i64>,
    #[serde(rename = "o.awen")]
    pub(crate) o_awen: Option<i64>,
    #[serde(rename = "o.epiphany")]
    pub(crate) o_epiphany: Option<i64>,
    #[serde(rename = "o.duende")]
    pub(crate) o_duende: Option<i64>,
    #[serde(rename = "o.ubisunt")]
    pub(crate) o_ubisunt: Option<i64>,
    pub(crate) opportunity: Option<i64>,
    pub(crate) zcho: Option<i64>,
    pub(crate) heart: Option<Edge>,
    pub(crate) zere: Option<i64>,
    pub(crate) nectar: Option<Edge>,
    pub(crate) zfet: Option<i64>,
    pub(crate) zhea: Option<i64>,
    pub(crate) winter: Option<Edge>,
    pub(crate) zpho: Option<i64>,
    pub(crate) lantern: Option<Edge>,
    pub(crate) zsha: Option<i64>,
    pub(crate) ztri: Option<i64>,
    pub(crate) edge: Option<Edge>,
    pub(crate) zwis: Option<i64>,
    pub(crate) scale: Option<Edge>,
    #[serde(rename = "w.cracktrack")]
    pub(crate) w_cracktrack: Option<i64>,
    #[serde(rename = "s.cracktrack")]
    pub(crate) s_cracktrack: Option<i64>,
    #[serde(rename = "w.ericapaean")]
    pub(crate) w_ericapaean: Option<i64>,
    #[serde(rename = "s.ericapaean")]
    pub(crate) s_ericapaean: Option<i64>,
    #[serde(rename = "w.fucine")]
    pub(crate) w_fucine: Option<i64>,
    #[serde(rename = "s.fucine")]
    pub(crate) s_fucine: Option<i64>,
    #[serde(rename = "w.henavek")]
    pub(crate) w_henavek: Option<i64>,
    #[serde(rename = "s.henavek")]
    pub(crate) s_henavek: Option<i64>,
    #[serde(rename = "w.hyksos")]
    pub(crate) w_hyksos: Option<i64>,
    #[serde(rename = "s.hyksos")]
    pub(crate) s_hyksos: Option<i64>,
    #[serde(rename = "w.killasimi")]
    pub(crate) w_killasimi: Option<i64>,
    #[serde(rename = "s.killasimi")]
    pub(crate) s_killasimi: Option<i64>,
    #[serde(rename = "w.mandaic")]
    pub(crate) w_mandaic: Option<i64>,
    #[serde(rename = "s.mandaic")]
    pub(crate) s_mandaic: Option<i64>,
    #[serde(rename = "w.ramsund")]
    pub(crate) w_ramsund: Option<i64>,
    #[serde(rename = "s.ramsund")]
    pub(crate) s_ramsund: Option<i64>,
    #[serde(rename = "w.sabazine")]
    pub(crate) w_sabazine: Option<i64>,
    #[serde(rename = "s.sabazine")]
    pub(crate) s_sabazine: Option<i64>,
    #[serde(rename = "w.vak")]
    pub(crate) w_vak: Option<i64>,
    #[serde(rename = "s.vak")]
    pub(crate) s_vak: Option<i64>,
    #[serde(rename = "mastery.edge")]
    pub(crate) mastery_edge: Option<Edge>,
    pub(crate) journal: Option<i64>,
    #[serde(rename = "mystery.forge")]
    pub(crate) mystery_forge: Option<i64>,
    #[serde(rename = "mastery.forge")]
    pub(crate) mastery_forge: Option<Edge>,
    pub(crate) forge: Option<Edge>,
    #[serde(rename = "mastery.grail")]
    pub(crate) mastery_grail: Option<Edge>,
    pub(crate) grail: Option<Edge>,
    #[serde(rename = "mystery.heart")]
    pub(crate) mystery_heart: Option<i64>,
    #[serde(rename = "mastery.heart")]
    pub(crate) mastery_heart: Option<Edge>,
    #[serde(rename = "mystery.knock")]
    pub(crate) mystery_knock: Option<i64>,
    #[serde(rename = "mastery.knock")]
    pub(crate) mastery_knock: Option<Edge>,
    #[serde(rename = "mastery.lantern")]
    pub(crate) mastery_lantern: Option<Edge>,
    #[serde(rename = "mystery.moon")]
    pub(crate) mystery_moon: Option<i64>,
    #[serde(rename = "mastery.moon")]
    pub(crate) mastery_moon: Option<Edge>,
    pub(crate) moon: Option<Edge>,
    #[serde(rename = "mystery.moth")]
    pub(crate) mystery_moth: Option<i64>,
    #[serde(rename = "mastery.moth")]
    pub(crate) mastery_moth: Option<Edge>,
    pub(crate) moth: Option<Edge>,
    #[serde(rename = "mystery.nectar")]
    pub(crate) mystery_nectar: Option<i64>,
    #[serde(rename = "mastery.nectar")]
    pub(crate) mastery_nectar: Option<Edge>,
    #[serde(rename = "mystery.rose")]
    pub(crate) mystery_rose: Option<i64>,
    #[serde(rename = "mastery.rose")]
    pub(crate) mastery_rose: Option<Edge>,
    #[serde(rename = "mystery.scale")]
    pub(crate) mystery_scale: Option<i64>,
    #[serde(rename = "mastery.scale")]
    pub(crate) mastery_scale: Option<Edge>,
    #[serde(rename = "mystery.sky")]
    pub(crate) mystery_sky: Option<i64>,
    #[serde(rename = "mastery.sky")]
    pub(crate) mastery_sky: Option<Edge>,
    #[serde(rename = "mastery.winter")]
    pub(crate) mastery_winter: Option<Edge>,
    pub(crate) film: Option<i64>,
    pub(crate) correspondence: Option<i64>,
    #[serde(rename = "record.phonograph")]
    pub(crate) record_phonograph: Option<i64>,
    #[serde(rename = "form.order")]
    pub(crate) form_order: Option<i64>,
    pub(crate) ink: Option<i64>,
    #[serde(rename = "de.edge")]
    pub(crate) de_edge: Option<i64>,
    #[serde(rename = "de.forge")]
    pub(crate) de_forge: Option<i64>,
    #[serde(rename = "de.grail")]
    pub(crate) de_grail: Option<i64>,
    #[serde(rename = "de.heart")]
    pub(crate) de_heart: Option<i64>,
    #[serde(rename = "de.knock")]
    pub(crate) de_knock: Option<i64>,
    #[serde(rename = "de.lantern")]
    pub(crate) de_lantern: Option<i64>,
    #[serde(rename = "de.moon")]
    pub(crate) de_moon: Option<i64>,
    #[serde(rename = "de.moth")]
    pub(crate) de_moth: Option<i64>,
    #[serde(rename = "de.nectar")]
    pub(crate) de_nectar: Option<i64>,
    #[serde(rename = "de.rose")]
    pub(crate) de_rose: Option<i64>,
    #[serde(rename = "de.scale")]
    pub(crate) de_scale: Option<i64>,
    #[serde(rename = "de.sky")]
    pub(crate) de_sky: Option<i64>,
    #[serde(rename = "de.winter")]
    pub(crate) de_winter: Option<i64>,
    #[serde(rename = "mem.fear")]
    pub(crate) mem_fear: Option<i64>,
    pub(crate) skill: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub(crate) id: Option<String>,
    pub(crate) label: Option<String>,
    pub(crate) required: Option<Required>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Required {
    pub(crate) ability: Option<i64>,
    pub(crate) weather: Option<i64>,
    pub(crate) memory: Option<i64>,
    pub(crate) comfort: Option<i64>,
    pub(crate) thing: Option<i64>,
    pub(crate) wallart: Option<i64>,
}

impl Identifiable for Element {
    // TODO: revisit this function to ensure query interface is intuitive
    //  - Currently, id will match entries such as "study.book.general.hint"
    //  - We might want to use label, e.g. "Mystery and Mastery?"
    fn id(&self) -> &str {
        &self.id
    }
    fn inner_id(&self) -> &str {
        &self.id
    }
}

impl FindById for ConsiderBooks {
    type Item = Element;

    type Collection = Vec<Element>;

    #[tracing::instrument(skip(self))]
    fn get_collection(&self) -> &Self::Collection {
        self.elements.get_collection()
    }
}

impl From<Value> for ConsiderBooks {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl GameCollectionType for ConsiderBooks {
    fn get_collection_type(&self) -> QueryType {
        QueryType::ConsiderBooks
    }
}

impl GameElementDetails for Element {
    fn get_label(&self) -> String {
        let a = self.clone().label;
        let b = a.unwrap_or_default();
        b.clone()
    }
    fn get_desc(&self) -> String {
        let a = self.clone().desc;
        let b = a.unwrap_or_default();
        b.clone()
    }
}
