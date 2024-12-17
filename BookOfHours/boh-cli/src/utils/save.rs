use std::fmt::Debug;
use anyhow::{anyhow, bail, Result};
use anyhow::Ok;
use clipboard_rs::{Clipboard, ClipboardContext};
use tracing::{debug, error, trace, warn};
use crate::model::Identifiable;
use crate::model::save::{Autosave, PayloadType, RootPopulationCommandSphere};

impl Autosave {

    #[tracing::instrument(skip(self))]
    pub(crate) fn check_if_item_manifested<T>(
        &self,
        game_item: &T
    ) -> anyhow::Result<bool>
    where
        T: Identifiable + Debug,
    {
        let manifested_items = self.get_unique_items()?;
        let item_id = game_item.inner_id().to_string();
        debug!(
            ?item_id,
            "Checking if item has been manifested"
        );
        Ok(manifested_items.contains(&item_id))
    }

    #[tracing::instrument(skip(self, game_item))]
    pub(crate) fn get_item_from_save_file<T>(
        &self,
        game_item: &T,
    ) -> anyhow::Result<RootPopulationCommandSphere>
    where
        T: Identifiable + Debug,
    {
        // root_population_command
        //  spheres
        //   tokens
        //    payload
        //     entity_id

        trace!(
            ?game_item,
            "Getting item from save file"
        );

        let clip = ClipboardContext::new().unwrap();
        let serd = serde_json::to_string(&self)?;
        clip.set_text(serd).unwrap();

        if let Some(root_population_command) = self.root_population_command.as_ref().clone() {
            if let Some(spheres) = &root_population_command.spheres {
                let a = spheres.iter().find(|sphere| {
                    let sphere_spec = sphere.governing_sphere_spec.as_ref().unwrap();
                    let sphere_id = sphere_spec.id.as_ref().unwrap();
                    debug!(
                        sphere_name =? sphere_id,
                        game_item_id =? game_item.inner_id(),
                        "Checking if sphere contains item"
                    );
                    if let Some(tokens) = &sphere.tokens {
                        tokens.iter().find(|token| {
                            if let Some(payload) = &token.payload {
                                trace!(
                                    payload_id =? payload.id,
                                    game_item_id =? game_item.inner_id(),
                                    "Checking payload type"
                                );
                                if payload.payload_type == PayloadType::ElementStackCreationCommand {
                                    if let Some(payload_id) = &payload.id {
                                        trace!(
                                            ?payload_id,
                                            game_item_id =? game_item.inner_id(),
                                            "Checking if id contains game_item ID"
                                        );
                                        payload_id.to_ascii_lowercase().contains(game_item.inner_id().to_ascii_lowercase().as_str())
                                        // game_item.inner_id().eq(entity_id)
                                    } else {
                                        // anyhow!(Err("Failed to find entity_id in payload"))
                                        error!(
                                            id =? payload.clone().id.unwrap_or_default(),
                                            sphere_name =? sphere_id,
                                            "Failed to find id in payload"
                                        );
                                        false
                                    }
                                } else {
                                    // anyhow!(Err("Failed to find payload in token"))
                                    // error!("Failed to find payload in token");
                                    warn!(
                                        payload_type =? payload.payload_type,
                                        sphere_name =? sphere_id,
                                        "Not processing token because payload is not ElementStackCreationCommand"
                                    );
                                    false
                                }
                            } else {
                                warn!("Skipping token because payload is None");
                                false
                            }
                        }).is_some()
                    } else {
                        // anyhow!(Err("Failed to find tokens in sphere"))
                        // bail!("Failed to find tokens in sphere")
                        error!("Failed to find tokens in sphere");
                        false
                    }
                });
                let b = a.cloned();
                match b {
                    Some(sphere) => Ok(sphere),
                    None => bail!("Failed to find sphere with item")
                }
                //.cloned().ok_or_else(|| bail!("Failed to find sphere with item"))
            }  else {
                 bail!("Failed to find spheres in root population command")
                 // error!("Failed to find spheres in root population command");
                 // None
            }
        } else {
            // anyhow!(Err("Failed to find root population command in save file"))
            bail!("Failed to find root population command in save file")
            // error!("Failed to find root population command in save file");
            // None
        }
    }



    pub(crate) fn get_unique_items(&self) -> anyhow::Result<Vec<String>> {
        assert!(
            self.character_creation_commands.is_some(),
            "Character creation commands should be present"
        );

        let character_creation_commands = self
            .clone()
            .character_creation_commands
            .expect("Character creation commands should be present");

        let unique_items_manifested = &character_creation_commands.first()
            .expect("Failed to get first item in character creation commands")
            .unique_elements_manifested;

        assert!(unique_items_manifested.is_some(), "Unique items should be present");

        let unique_items = unique_items_manifested.as_ref().unwrap();

        Ok(unique_items.to_vec())
    }
}