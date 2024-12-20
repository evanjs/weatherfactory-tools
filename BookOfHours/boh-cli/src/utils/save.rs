use crate::model::save::{
    Autosave, PayloadType, StickyPayload, TentacledPayload,
};
use crate::model::Identifiable;
use anyhow::{anyhow, Ok};
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


        // Find the sphere containing the item
        for root_population_command_sphere in &self.root_population_command.spheres {
            for fluffy_token in &root_population_command_sphere.tokens {
                for purple_dominion in &fluffy_token.payload.dominions {
                    for purple_sphere in &purple_dominion.spheres {
                        for tentacled_token in &purple_sphere.tokens {
                            let payload = &tentacled_token.payload;
                            if tentacled_token.payload.payload_type == PayloadType::ElementStackCreationCommand {
                                trace!(
                                    payload_id =? payload.id,
                                    game_item_id =? game_item.inner_id(),
                                    "Checking if payload contains item"
                                );
                                if payload.id
                                    .to_ascii_lowercase()
                                    .contains(game_item
                                        .inner_id()
                                        .to_ascii_lowercase()
                                        .as_str())
                                {
                                    // Return the matching innermost payload
                                    return Ok(tentacled_token.payload.clone());
                                }
                            } else {
                                trace!(
                                    dpayload_id=? tentacled_token.payload.id,
                                    "Payload type is not ElementStackCreationCommand"
                                );
                            }
                        }
                    }
                }
            }
        }

        bail!("Could not find item in save file")
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

        debug!("Found item in save file.\nChecking if item is mastered or being studied");
        if mastered_item.is_err() {
            let studying_item = self.get_studying_item_from_save_file(game_item);
            debug!("Item is not mastered, checking if it is being studied");
            Either::Left(studying_item)
        } else {
            debug!("Found mastered item, returning it instead of the studying item");
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

        // Find the sphere containing the item
        for root_population_command_sphere in &self.root_population_command.spheres {
            for fluffy_token in &root_population_command_sphere.tokens {
                for purple_dominion in &fluffy_token.payload.dominions {
                    for purple_sphere in &purple_dominion.spheres {
                        for tentacled_token in &purple_sphere.tokens {
                            if tentacled_token.payload.payload_type == PayloadType::SituationCreationCommand {
                                for fluffy_dominion in &tentacled_token.payload.dominions {
                                    for fluffy_sphere in &fluffy_dominion.spheres {
                                        for sticky_token in &fluffy_sphere.tokens {
                                            trace!(
                                                payload_id =? sticky_token.payload.id,
                                                game_item_id =? game_item.inner_id(),
                                                "Checking if payload contains item"
                                            );
                                            if sticky_token.payload.id
                                                .to_ascii_lowercase()
                                                .contains(game_item
                                                    .inner_id()
                                                    .to_ascii_lowercase()
                                                    .as_str())
                                            {
                                                // Return the matching innermost payload
                                                return Ok(sticky_token.payload.clone());
                                            }
                                        }
                                    }
                                }
                            }  else {
                                trace!(
                                    dpayload_id =? tentacled_token.payload.id,
                                    dpayload_type =? tentacled_token.payload.payload_type,
                                    "Payload type is not ElementStackCreationCommand"
                                );
                            }
                        }
                    }
                }
            }
        }

        bail!("Could not find item in save file")
    }

pub(crate) fn get_unique_items(&self) -> anyhow::Result<Vec<String>> {
    let character_creation_commands = self
        .clone()
        .character_creation_commands;

    let unique_items_manifested = &character_creation_commands
        .first()
        .expect("Failed to get first item in character creation commands")
        .unique_elements_manifested;

        let unique_items = unique_items_manifested;

        Ok(unique_items.to_vec())
    }
}
