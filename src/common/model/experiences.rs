use crate::features::home::components::ExperienceCardProps;
use dioxus::{asset_resolver::read_asset_bytes, prelude::*};

const EXPERIENCES: Asset = asset!("./assets/experiences.json");

pub async fn get_experiences() -> dioxus::Result<Vec<ExperienceCardProps>> {
    Ok(serde_json::from_slice(
        &read_asset_bytes(&EXPERIENCES).await?,
    )?)
}
