use std::collections::HashMap;

use crate::{
    common::model::experiences::get_experiences,
    features::home::components::experience_card::ExperienceCard,
};
use dioxus::{logger::tracing, prelude::*};

pub const EXPERIENCES_SECTION_ID: &str = "experiences";

#[component]
pub fn Experiences(elements: Signal<HashMap<&'static str, Event<MountedData>>>) -> Element {
    let experiences = use_resource(get_experiences);

    rsx! {
        section {
            id: EXPERIENCES_SECTION_ID,
            class: "py-20",
            onmounted: move |element| {
                elements.write().insert(EXPERIENCES_SECTION_ID, element);
            },
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Experience"
                }
                div { class: "flex flex-col max-w-3xl mx-auto space-y-8",
                    match &*experiences.read() {
                        Some(Ok(experiences)) => rsx! {
                            for experience in experiences.clone() {
                                ExperienceCard { ..experience }
                            }
                        },
                        Some(Err(err)) => {
                            tracing::debug!(? err, "Failed to load blog posts");
                            rsx! { "Loading project posts failed" }
                        }
                        None => rsx! { "Loading blogs posts ..." },
                    }
                }
            }
        }
    }
}
