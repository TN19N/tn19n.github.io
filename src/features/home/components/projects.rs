use std::collections::HashMap;

use crate::{
    common::model::projects::get_projects, features::home::components::project_card::ProjectCard,
};
use dioxus::{core::throw_error, prelude::*};

pub const PROJECTS_SECTION_ID: &str = "projects";

#[component]
pub fn Projects(elements: Signal<HashMap<&'static str, Event<MountedData>>>) -> Element {
    let projects = use_resource(get_projects);

    rsx! {
        section {
            id: PROJECTS_SECTION_ID,
            class: "py-20",
            onmounted: move |element| {
                elements.write().insert(PROJECTS_SECTION_ID, element);
            },
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Featured Projects"
                }
                div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                    match &*projects.read() {
                        Some(Ok(projects)) => rsx! {
                            for project in projects.clone() {
                                ProjectCard { ..project }
                            }
                        },
                        Some(Err(err)) => {
                            throw_error(err.clone());
                            rsx! { "Loading blog posts failed" }
                        }
                        None => rsx! { "Loading blogs posts ..." },
                    }
                }
            }
        }
    }
}
