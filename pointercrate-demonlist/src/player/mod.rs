pub use self::{
    paginate::{PlayerPagination, RankingPagination},
    patch::PatchPlayer,
};
use crate::{demon::MinimalDemon, error::Result, nationality::Nationality, record::MinimalRecordD};
use derive_more::Display;
use pointercrate_core::etag::Taggable;
use serde::Serialize;
use sqlx::PgConnection;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub mod claim;
mod get;
mod paginate;
mod patch;

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Display, Clone)]
#[display(fmt = "{} (ID: {})", name, id)]
pub struct DatabasePlayer {
    pub id: i32,
    pub name: String,
    pub banned: bool,
}

#[derive(Debug, Serialize, Display, PartialEq, Eq, Hash)]
#[display(fmt = "{}", player)]
pub struct FullPlayer {
    #[serde(flatten)]
    pub player: Player,
    pub records: Vec<MinimalRecordD>,
    pub created: Vec<MinimalDemon>,
    pub verified: Vec<MinimalDemon>,
    pub published: Vec<MinimalDemon>,
}

#[derive(Debug, PartialEq, Serialize, Display)]
#[display(fmt = "{} (ID: {}) at rank {} with score {}", name, id, rank, score)]
pub struct RankedPlayer {
    pub id: i32,
    pub name: String,
    pub rank: i64,
    pub score: f64,
    pub nationality: Option<Nationality>,
    #[serde(skip)]
    pub index: i64,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Display)]
#[display(fmt = "{}", base)]
pub struct Player {
    #[serde(flatten)]
    pub base: DatabasePlayer,

    pub nationality: Option<Nationality>,
}

impl Taggable for FullPlayer {
    fn patch_part(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.player.hash(&mut hasher);
        hasher.finish()
    }
}

impl RankedPlayer {
    /// Gets the highest index value generated by the `players_with_score` view
    pub async fn max_index(connection: &mut PgConnection) -> Result<i64> {
        Ok(
            sqlx::query!(r#"SELECT COALESCE(MAX(index), 0) AS "max_index!: i64" FROM players_with_score"#)
                .fetch_one(connection)
                .await?
                .max_index,
        )
    }
}

impl Player {
    /// Gets the maximal and minimal player id currently in use
    ///
    /// The returned tuple is of the form (max, min)
    pub async fn extremal_player_ids(connection: &mut PgConnection) -> Result<(i32, i32)> {
        let row = sqlx::query!(r#"SELECT COALESCE(MAX(id), 0) AS "max_id!: i32", COALESCE(MIN(id), 0) AS "min_id!: i32" FROM players"#)
            .fetch_one(connection)
            .await?;
        Ok((row.max_id, row.min_id))
    }
}
