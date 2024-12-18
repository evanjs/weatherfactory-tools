use crate::model::save::{
    Autosave, PayloadType, StickyPayload, TentacledPayload,
};
use crate::model::Identifiable;
use anyhow::Ok;
use anyhow::bail;
use either::Either;
use std::fmt::Debug;
use tracing::{debug, trace, warn};

impl Autosave {
    #[tracing::instrument(skip(self))]
    pub(crate) fn check_if_item_manifested<T>(&self, game_item: &T) -> anyhow::Result<bool>
    where
        T: Identifiable + Debug,
    {
        let manifested_items = self.get_unique_items()?;
        let item_id = game_item.inner_id().to_string();
        debug!(?item_id, "Checking if item has been manifested");
        Ok(manifested_items.contains(&item_id))
    }

    #[tracing::instrument(skip(self, game_item))]
    pub(crate) fn get_item_from_save_file<T>(
        &self,
        game_item: &T,
    ) -> anyhow::Result<TentacledPayload>
    where
        T: Identifiable + Debug + ?Sized,
    {
        // root_population_command
        //  spheres
        //   tokens
        //    payload
        //     entity_id

        trace!(?game_item, "Getting item from save file");

        if let Some(root_population_command) = self.root_population_command.as_ref() {
            if let Some(spheres) = &root_population_command.spheres {
                // Find the sphere containing the item
                for sphere in spheres {
                    if let Some(tokens) = &sphere.tokens {
                        for token in tokens {
                            if let Some(payload) = &token.payload {
                                if let Some(dominions) = &payload.dominions {
                                    for dominion in dominions {
                                        if let Some(dspheres) = &dominion.spheres {
                                            for dsphere in dspheres {
                                                while let Some(dtokens) = &dsphere.tokens {
                                                    for dtoken in dtokens {
                                                        if let Some(dpayload) =
                                                            dtoken.payload.as_ref()
                                                        {
                                                            if dpayload.payload_type == PayloadType::ElementStackCreationCommand {
                                                                if let Some(dpayload_id) = &dpayload.id {
                                                                    debug!(
                                                                        payload_id =? dpayload_id,
                                                                        game_item_id =? game_item.inner_id(),
                                                                        "Checking if payload contains item"
                                                                    );
                                                                    if dpayload_id
                                                                        .to_ascii_lowercase()
                                                                        .contains(game_item
                                                                            .inner_id()
                                                                            .to_ascii_lowercase()
                                                                            .as_str())
                                                                    {
                                                                        // Return the matching innermost payload
                                                                        return Ok(dpayload.clone());
                                                                    }
                                                                }
                                                            } else {
                                                                trace!(
                                                                    dpayload =? dpayload.id,
                                                                    "Payload type is not ElementStackCreationCommand"
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                bail!("Failed to find matching item in any sphere");
            } else {
                bail!("Failed to find spheres in root population command");
            }
        } else {
            bail!("Failed to find root population command in save file");
        }
    }

    /// Determine whether the provided time is either mastered or currently being studied
    /// These criteria appear a bit differently in the save file
    ///
    /// StickyPayloads are nested more than TentacledPayloads
    ///
    /// # Arguments
    ///
    /// * `game_item`: the tome to query in the save file
    ///
    /// returns: Either<Result<StickyPayload, Error>, Result<TentacledPayload, Error>>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub(crate) fn get_mastered_or_studying_item_from_save_file<T>(
        &self,
        game_item: &T,
    ) -> Either<anyhow::Result<StickyPayload>, anyhow::Result<TentacledPayload>>
    where
        T: Identifiable + Debug + ?Sized,
    {
        let mastered_item = self.get_item_from_save_file(game_item);

        debug!(
            ?mastered_item,
            "Found item in save file.\nChecking if item is mastered or being studied"
        );
        if mastered_item.is_err() {
            let studying_item = self.get_studying_item_from_save_file(game_item);
            debug!(
                ?studying_item,
                "Item is not mastered, checking if it is being studied"
            );
            Either::Left(studying_item)
        } else {
            debug!(
                ?mastered_item,
                "Found mastered item, returning it instead of the studying item"
            );
            Either::Right(mastered_item)
        }
    }

    #[tracing::instrument(skip(self, game_item))]
    pub(crate) fn get_studying_item_from_save_file<T>(
        &self,
        game_item: &T,
    ) -> anyhow::Result<StickyPayload>
    where
        T: Identifiable + Debug + ?Sized,
    {
        // root_population_command
        //  spheres
        //   tokens
        //    payload
        //     entity_id

        trace!(?game_item, "Getting item from save file");

        if let Some(root_population_command) = self.root_population_command.as_ref() {
            if let Some(spheres) = &root_population_command.spheres {
                // Find the sphere containing the item
                for sphere in spheres {
                    if let Some(tokens) = &sphere.tokens {
                        for token in tokens {
                            if let Some(payload) = &token.payload {
                                if let Some(dominions) = &payload.dominions {
                                    for dominion in dominions {
                                        if let Some(dspheres) = &dominion.spheres {
                                            for dsphere in dspheres {
                                                while let Some(dtokens) = &dsphere.tokens {
                                                    for dtoken in dtokens {
                                                        if let Some(dpayload) =
                                                            dtoken.payload.as_ref()
                                                        {
                                                            if dpayload.payload_type == PayloadType::SituationCreationCommand {
                                                                if let Some(ddominions) = &dpayload.dominions {
                                                                    for ddominion in ddominions {
                                                                        if let Some(ddspheres) = &ddominion.spheres {
                                                                            for ddsphere in ddspheres {
                                                                                while let Some(ddtokens) = &ddsphere.tokens {
                                                                                    for ddtoken in ddtokens {
                                                                                        if let Some(ddpayload) = ddtoken.payload.as_ref() {
                                                                                            if let Some(ddpayload_id) = &ddpayload.id {
                                                                                                debug!(
                                                                                                    payload_id =? ddpayload_id,
                                                                                                    game_item_id =? game_item.inner_id(),
                                                                                                    "Checking if payload contains item"
                                                                                                );
                                                                                                if ddpayload_id
                                                                                                    .to_ascii_lowercase()
                                                                                                    .contains(game_item
                                                                                                        .inner_id()
                                                                                                        .to_ascii_lowercase()
                                                                                                        .as_str())
                                                                                                {
                                                                                                    // Return the matching innermost payload
                                                                                                    return Ok(ddpayload.clone());
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                trace!(
                                                                    dpayload =? dpayload.id,
                                                                    "Payload type is not ElementStackCreationCommand"
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                bail!("Failed to find matching item in any sphere");
            } else {
                bail!("Failed to find spheres in root population command");
            }
        } else {
            bail!("Failed to find root population command in save file");
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

        let unique_items_manifested = &character_creation_commands
            .first()
            .expect("Failed to get first item in character creation commands")
            .unique_elements_manifested;

        assert!(
            unique_items_manifested.is_some(),
            "Unique items should be present"
        );

        let unique_items = unique_items_manifested.as_ref().unwrap();

        Ok(unique_items.to_vec())
    }
}
