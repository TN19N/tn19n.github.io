use crate::{
    common::{
        components::{Badge, Card, CardContent, CardDescription, CardHeader, CardTitle},
        icons::{calendar::Calendar, external_link::ExternalLink},
        model::posts_metadata::PostMetadata,
    },
    router::AppRouter,
};
pub use dioxus::prelude::*;

#[component]
pub fn PostCard(post_metadata: PostMetadata) -> Element {
    rsx! {
        Link {
            to: AppRouter::Post {
                post: post_metadata.post.clone(),
            },
            Card {
                CardHeader {
                    CardTitle {
                        "{post_metadata.title}"
                        ExternalLink { class: "w-4 h-4 text-primary" }
                    }
                    CardDescription {
                        Calendar { class: "w-4 h-4" }
                        "{post_metadata.date.format(\"%b %d, %Y\")}"
                    }
                }
                CardContent {
                    p { class: "text-sm text-muted-foreground mb-4", "{post_metadata.description}" }
                    div { class: "flex flex-wrap gap-2",
                        for tag in post_metadata.tags.iter() {
                            Badge { "{tag}" }
                        }
                    }
                }
            }
        }
    }
}
