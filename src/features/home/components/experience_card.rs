use crate::common::{
    components::{Badge, BadgeVariant, Card, CardContent, CardDescription, CardHeader, CardTitle},
    icons::external_link::ExternalLink,
};
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ExperienceYearsDuration {
    pub from: u32,
    pub to: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Props, Deserialize)]
pub struct ExperienceCardProps {
    link: String,
    role: String,
    company: String,
    duration: ExperienceYearsDuration,
    tasks: Vec<String>,
}

#[component]
pub fn ExperienceCard(props: ExperienceCardProps) -> Element {
    rsx! {
        a { href: "{props.link}", target: "_blank",
            Card {
                CardHeader {
                    div { class: "flex justify-between items-start",
                        div {
                            CardTitle { "{props.role}" }
                            CardDescription { "{props.company}" }
                        }
                        div { class: "flex gap-5 items-center",
                            Badge {
                                class: "border-primary text-primary",
                                variant: BadgeVariant::Outline,
                                "{props.duration.from} - "
                                if let Some(year) = props.duration.to {
                                    "{year}"
                                } else {
                                    "Present"
                                }
                            }
                            ExternalLink { class: "w-4 h-4 text-primary" }
                        }
                    }
                }
                CardContent {
                    ul { class: "list-disc list-inside space-y-2 text-muted-foreground",
                        for task in props.tasks {
                            li { "{task}" }
                        }
                    }
                }
            }
        }
    }
}
