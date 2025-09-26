use crate::common::{
    components::{Badge, Card, CardContent, CardDescription, CardHeader, CardTitle},
    icons::external_link::ExternalLink,
};
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Props, Deserialize)]
pub struct ProjectCardProps {
    link: String,
    title: String,
    technologies: Vec<String>,
    description: String,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    rsx! {
        a { href: "{props.link}", target: "_blank",
            Card {
                CardHeader {
                    CardTitle {
                        "{props.title}"
                        ExternalLink { class: "w-4 h-4 text-primary" }
                    }
                    CardDescription { "{props.description}" }
                }
                CardContent {
                    div { class: "flex flex-wrap gap-2 mb-4",
                        for technologie in props.technologies {
                            Badge { "{technologie}" }
                        }
                    }
                }
            }
        }
    }
}
