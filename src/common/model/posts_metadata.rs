use chrono::{DateTime, Local};
use dioxus::{asset_resolver::read_asset_bytes, prelude::*};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PostMetadata {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub date: DateTime<Local>,
    pub post: String,
}

const POSTS_METADATA: Asset = asset!("./assets/blogs.json");

pub async fn get_posts_metadata() -> dioxus::Result<Vec<PostMetadata>> {
    Ok(
        serde_json::from_slice::<Vec<PostMetadata>>(&read_asset_bytes(&POSTS_METADATA).await?)
            .map(|mut posts_metadata| {
                posts_metadata.sort_by_key(|post_metadata| post_metadata.date);
                posts_metadata
            })?,
    )
}

pub async fn get_post_metadata_by_name(post: &str) -> dioxus::Result<Option<PostMetadata>> {
    Ok(get_posts_metadata()
        .await?
        .into_iter()
        .find(|post_metadata| post_metadata.post == post))
}
