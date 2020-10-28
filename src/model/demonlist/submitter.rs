pub use self::{paginate::SubmitterPagination, patch::PatchSubmitter};
use crate::Result;
use derive_more::Display;
use serde::Serialize;
use sqlx::PgConnection;

mod get;
mod paginate;
mod patch;

#[derive(Debug, Serialize, Hash, Display, Copy, Clone)]
#[display(fmt = "{} (Banned: {})", id, banned)]
pub struct Submitter {
    pub id: i32,
    pub banned: bool,
}

impl Submitter {
    /// Gets the maximal and minimal submitter id currently in use
    ///
    /// The returned tuple is of the form (max, min)
    pub async fn extremal_submitter_ids(connection: &mut PgConnection) -> Result<(i32, i32)> {
        let row = sqlx::query!("SELECT COALESCE(MAX(submitter_id), 0) AS max_id, COALESCE(MIN(submitter_id), 0) AS min_id FROM submitters")
            .fetch_one(connection)
            .await?;
        Ok((row.max_id, row.min_id))
    }
}
