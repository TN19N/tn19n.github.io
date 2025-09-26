use crate::{
    common::{
        components::{Button, ButtonVariant, PostCard},
        icons::arrow_left::ArrowLeft,
        model::posts_metadata::get_posts_metadata,
    },
    router::AppRouter,
};
use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn Blog() -> Element {
    let posts_metadata = use_resource(get_posts_metadata);

    rsx! {
        main { class: "flex-1",

            // Blog Header
            section { class: "py-16 bg-gradient-to-br from-background to-muted",
                div { class: "container mx-auto px-4",
                    div { class: "flex items-center gap-4 mb-6",
                        Link {
                            to: AppRouter::Home {
                                anchor: "".into(),
                            },
                            Button {
                                class: "h-9 px-4 py-2",
                                variant: ButtonVariant::Outline,
                                ArrowLeft { class: "w-4 h-4 mr-2" }
                                "Back to Portfolio"
                            }
                        }
                    }
                    h1 { class: "text-4xl font-bold text-primary mb-4", "Technical Blog" }
                    p { class: "text-lg text-muted-foreground max-w-2xl leading-relaxed",
                        "Insights, tutorials, and deep dives into backend development, system architecture, and engineering best practices."
                    }
                }
            }

            // Blog Posts
            section { class: "py-20",
                div { class: "container mx-auto px-4",
                    div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                        match &*posts_metadata.read() {
                            Some(Ok(posts_metadata)) => rsx! {
                                for post_metadata in posts_metadata {
                                    PostCard { post_metadata: post_metadata.clone() }
                                }
                            },
                            Some(Err(error)) => {
                                tracing::debug!(? error, "Failed to load blog posts");
                                rsx! { "Loading blog post failed" }
                            }
                            None => rsx! { "Loading blogs posts ..." },
                        }
                    }
                }
            }
        }
    }
}
