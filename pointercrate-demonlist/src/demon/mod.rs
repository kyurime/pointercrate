pub use self::{
    get::{current_list, list_at, published_by, verified_by},
    paginate::{DemonIdPagination, DemonPositionPagination},
    patch::PatchDemon,
    post::PostDemon,
};
use crate::{
    error::{DemonlistError, Result},
    player::DatabasePlayer,
    record::MinimalRecordP,
};
use derive_more::Display;
use log::info;
use pointercrate_core::{error::CoreError, etag::Taggable};
use serde::Serialize;
use sqlx::PgConnection;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[macro_use]
mod get;
pub mod audit;
mod paginate;
mod patch;
mod post;

pub struct TimeShiftedDemon {
    pub current_demon: Demon,
    pub position_now: i16,
}

/// Struct modelling a demon. These objects are returned from the paginating `/demons/` endpoint
#[derive(Debug, Serialize, Hash, Display, Eq, PartialEq)]
#[display(fmt = "{}", base)]
pub struct Demon {
    #[serde(flatten)]
    pub base: MinimalDemon,

    /// The minimal progress a [`Player`] must achieve on this [`Demon`] to have their record
    /// accepted
    pub requirement: i16,

    pub video: Option<String>,

    /// This [`Demon`]'s publisher
    pub publisher: DatabasePlayer,

    /// This [`Demon`]'s verifier
    pub verifier: DatabasePlayer,

    /// This ['Demons']'s Geometry Dash level ID
    ///
    /// This is automatically queried based on the level name, but can be manually overridden by a
    /// list mod.
    pub level_id: Option<u64>,
}

/// Absolutely minimal representation of a demon to be sent when a demon is part of another object
#[derive(Debug, Hash, Serialize, Display, PartialEq, Eq, Clone)]
#[display(fmt = "{} (at {})", name, position)]
pub struct MinimalDemon {
    /// The [`Demon`]'s unique internal pointercrate ID
    pub id: i32,

    /// The [`Demon`]'s position on the demonlist
    ///
    /// Positions for consecutive demons are always consecutive positive integers
    pub position: i16,

    /// The [`Demon`]'s Geometry Dash level name
    ///
    /// Note that the name doesn't need to be unique!
    pub name: String,
}

/// Struct modelling the "full" version of a demon.
///
/// In addition to containing publisher/verifier information it also contains a list of the demon's
/// creators and a list of accepted records
#[derive(Debug, Serialize, Display, PartialEq, Eq, Hash)]
#[display(fmt = "{}", demon)]
pub struct FullDemon {
    #[serde(flatten)]
    pub demon: Demon,
    pub creators: Vec<DatabasePlayer>,
    pub records: Vec<MinimalRecordP>,
}

impl Taggable for FullDemon {
    fn patch_part(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.demon.hash(&mut hasher);
        hasher.finish()
    }
}

impl MinimalDemon {
    /// Queries the record requirement for this demon from the database without collecting any of
    /// the other data
    pub async fn requirement(&self, connection: &mut PgConnection) -> Result<i16> {
        Ok(sqlx::query!("SELECT requirement FROM demons WHERE id = $1", self.id)
            .fetch_one(connection)
            .await?
            .requirement)
    }
}

impl FullDemon {
    pub fn position(&self) -> i16 {
        self.demon.base.position
    }

    pub fn name(&self) -> &str {
        self.demon.base.name.as_ref()
    }

    pub fn headline(&self) -> String {
        let publisher = &self.demon.publisher.name;
        let verifier = &self.demon.verifier.name;

        let creator = match &self.creators[..] {
            [] => "Unknown".to_string(),
            [creator] => creator.name.to_string(),
            many => {
                let mut iter = many.iter();
                let fst = iter.next().unwrap();

                format!(
                    "{} and {}",
                    iter.map(|player| player.name.to_string()).collect::<Vec<_>>().join(", "),
                    fst.name
                )
            },
        };

        // no comparison between &String and String, so just make it a reference
        let creator = &creator;

        if creator == verifier && creator == publisher {
            format!("by {}", creator)
        } else if creator != verifier && verifier == publisher {
            format!("by {}, verified and published by {}", creator, verifier)
        } else if creator != verifier && creator != publisher && publisher != verifier {
            format!("by {}, verified by {}, published by {}", creator, verifier, publisher)
        } else if creator == verifier && creator != publisher {
            format!("by {}, published by {}", creator, publisher)
        } else if creator == publisher && creator != verifier {
            format!("by {}, verified by {}", creator, verifier)
        } else {
            "If you're seeing this, file a bug report".to_string()
        }
    }

    pub fn short_headline(&self) -> String {
        let demon = &self.demon;

        if demon.publisher == demon.verifier {
            format!("verified and published by {}", demon.verifier.name)
        } else {
            format!("published by {}, verified by {}", demon.publisher.name, demon.verifier.name)
        }
    }
}

impl Demon {
    pub fn validate_requirement(requirement: i16) -> Result<()> {
        if !(0..=100).contains(&requirement) {
            return Err(DemonlistError::InvalidRequirement);
        }

        Ok(())
    }

    pub async fn validate_position(position: i16, connection: &mut PgConnection) -> Result<()> {
        let maximal_position = Demon::max_position(connection).await?;

        if position > maximal_position || position < 1 {
            return Err(DemonlistError::InvalidPosition { maximal: maximal_position });
        }

        Ok(())
    }

    /// Increments the position of all demons with positions equal to or greater than the given one,
    /// by one.
    async fn shift_down(starting_at: i16, connection: &mut PgConnection) -> Result<()> {
        info!("Shifting down all demons, starting at {}", starting_at);

        sqlx::query!("UPDATE demons SET position = position + 1 WHERE position >= $1", starting_at)
            .execute(connection)
            .await?;

        Ok(())
    }

    /// Decrements the position of all demons with positions equal to or smaller than the given one,
    /// by one.
    async fn shift_up(until: i16, connection: &mut PgConnection) -> Result<()> {
        info!("Shifting up all demons until {}", until);

        sqlx::query!("UPDATE demons SET position = position - 1 WHERE position <= $1", until)
            .execute(connection)
            .await?;

        Ok(())
    }

    /// Gets the current max position a demon has
    pub async fn max_position(connection: &mut PgConnection) -> Result<i16> {
        sqlx::query!("SELECT COALESCE(MAX(position), 1::SMALLINT) as max_position FROM demons")
            .fetch_one(connection)
            .await?
            .max_position
            .ok_or(CoreError::NotFound.into())
    }

    /// Gets the maximal and minimal submitter id currently in use
    ///
    /// The returned tuple is of the form (max, min)
    pub async fn extremal_demon_ids(connection: &mut PgConnection) -> Result<(i32, i32)> {
        let row = sqlx::query!(r#"SELECT COALESCE(MAX(id), 0) AS "max_id!: i32", COALESCE(MIN(id), 0) AS "min_id!: i32" FROM demons"#)
            .fetch_one(connection)
            .await?;
        Ok((row.max_id, row.min_id))
    }

    pub fn score(&self, progress: i16) -> f64 {
        let mut score = 100f64 * f64::exp((1f64 - f64::from(self.base.position)) * (1f64 / 30f64).ln() / (-99f64));

        if progress != 100 {
            score *= 0.25f64 + (f64::from(progress) - f64::from(self.requirement)) / (100f64 - f64::from(self.requirement)) * 0.25f64
        }

        score
    }
}
