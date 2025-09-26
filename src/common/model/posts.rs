use dioxus::prelude::*;
use gloo_net::http::Request;

const POSTS: Asset = asset!("./posts");

pub async fn get_post_by_name(post: &str) -> dioxus::Result<String> {
    Ok(Request::get(POSTS.resolve().join(post).to_str().unwrap())
        .send()
        .await?
        .text()
        .await?)
}
