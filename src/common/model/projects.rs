use crate::features::home::components::ProjectCardProps;
use dioxus::{asset_resolver::read_asset_bytes, prelude::*};

const PROJECTS: Asset = asset!("./assets/projects.json");

pub async fn get_projects() -> dioxus::Result<Vec<ProjectCardProps>> {
    Ok(serde_json::from_slice(&read_asset_bytes(&PROJECTS).await?)?)
}
