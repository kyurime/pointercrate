use crate::{
    creator::Creator,
    demon::{Demon, FullDemon, MinimalDemon},
    error::Result,
    player::DatabasePlayer,
};
use log::info;
use serde::Deserialize;
use sqlx::PgConnection;

#[derive(Deserialize, Debug)]
pub struct PostDemon {
    name: String,
    position: i16,
    requirement: i16,
    verifier: String,
    publisher: String,
    creators: Vec<String>,
    video: Option<String>,
}

impl FullDemon {
    /// Must be run within a transaction!
    pub async fn create_from(data: PostDemon, connection: &mut PgConnection) -> Result<FullDemon> {
        info!("Creating new demon from {:?}", data);

        Demon::validate_requirement(data.requirement)?;

        let video = match data.video {
            Some(ref video) => Some(crate::video::validate(video)?),
            None => None,
        };

        Demon::validate_position(data.position, connection).await?;

        let publisher = DatabasePlayer::by_name_or_create(data.publisher.as_ref(), connection).await?;
        let verifier = DatabasePlayer::by_name_or_create(data.verifier.as_ref(), connection).await?;

        Demon::shift_down(data.position, connection).await?;

        let created = sqlx::query!(
            "INSERT INTO demons (name, position, requirement, video, verifier, publisher) VALUES ($1::text,$2,$3,$4::text,$5,$6) \
             RETURNING id, thumbnail",
            data.name.to_string(),
            data.position,
            data.requirement,
            video.as_ref(),
            verifier.id,
            publisher.id
        )
        .fetch_one(&mut *connection)
        .await?;

        let demon = Demon {
            base: MinimalDemon {
                id: created.id,
                position: data.position,
                name: data.name,
            },
            requirement: data.requirement,
            video,
            thumbnail: created.thumbnail,
            publisher,
            verifier,
            level_id: None,
        };

        let mut creators = Vec::new();

        for creator in data.creators {
            let player = DatabasePlayer::by_name_or_create(creator.as_ref(), &mut *connection).await?;
            Creator::insert(&demon.base, &player, connection).await?;

            creators.push(player);
        }

        Ok(FullDemon {
            demon,
            creators,
            records: Vec::new(),
        })
    }
}
