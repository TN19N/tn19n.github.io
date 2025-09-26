use std::collections::HashMap;

use crate::{
    common::{
        components::{Button, ButtonVariant, PostCard},
        model::posts_metadata::get_posts_metadata,
    },
    router::AppRouter,
};
use dioxus::{logger::tracing, prelude::*};

pub const BLOG_SECTION_ID: &str = "blog";

#[component]
pub fn Blog(elements: Signal<HashMap<&'static str, Event<MountedData>>>) -> Element {
    let posts_metadata = use_resource(get_posts_metadata);

    rsx! {
        section {
            id: BLOG_SECTION_ID,
            class: "py-20",
            onmounted: move |element| {
                elements.write().insert(BLOG_SECTION_ID, element);
            },
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Latest Blog Posts"
                }
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                    match &*posts_metadata.read() {
                        Some(Ok(posts_metadata)) => rsx! {
                            for post_metadata in posts_metadata.iter().take(6) {
                                PostCard { post_metadata: post_metadata.clone() }
                            }
                        },
                        Some(Err(error)) => {
                            tracing::debug!(? error, "Failed to load blog posts");
                            rsx! { "Loading blog posts failed" }
                        }
                        None => rsx! { "Loading blogs posts ..." },
                    }
                }
                div { class: "text-center mt-12",
                    Link { to: AppRouter::Blog {},
                        Button {
                            class: "h-9 px-4 py-2",
                            variant: ButtonVariant::Outline,
                            "View All Posts"
                        }
                    }
                }
            }
        }
    }
}
