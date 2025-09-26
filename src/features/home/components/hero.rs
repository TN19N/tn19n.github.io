use crate::common::{
    components::{Button, ButtonVariant},
    icons::{download::Download, github::GitHub, mail::Mail},
};
use dioxus::prelude::*;
use std::collections::HashMap;

const CV: Asset = asset!("./assets/cv.pdf");
pub const HERO_SECTION_ID: &str = "hero";

#[component]
pub fn Hero(elements: Signal<HashMap<&'static str, Event<MountedData>>>) -> Element {
    rsx! {
        section {
            onmounted: move |element| {
                elements.write().insert(HERO_SECTION_ID, element);
            },
            id: HERO_SECTION_ID,
            class: "py-20 bg-gradient-to-br from-background to-muted",
            div { class: "container mx-auto px-4 text-center",
                h2 { class: "text-3xl sm:text-5xl font-bold text-primary mb-4", "Backend Developer" }
                p { class: "text-xl text-primary font-medium mb-6",
                    "Building scalable systems & robust APIs"
                }
                p { class: "text-lg text-muted-foreground max-w-2xl mx-auto mb-8 leading-relaxed",
                    "Passionate about creating efficient, maintainable backend solutions. Experienced in microservices architecture, database optimization, and cloud infrastructure with a focus on performance and reliability."
                }
                div { class: "flex flex-col sm:flex-row items-center justify-center gap-4",
                    a { href: "#contact",
                        Button { class: "h-9 px-4 py-2",
                            Mail { class: "w-4 h-4 mr-2" }
                            "Get In touch"
                        }
                    }
                    a { href: "https://github.com/TN19N", target: "_blank",
                        Button {
                            variant: ButtonVariant::Outline,
                            class: "h-9 px-4 py-2",
                            GitHub { class: "w-4 h-4 mr-2" }
                            "View Github"
                        }
                    }

                    a { href: CV, download: "TN19N_CV.pdf",
                        Button {
                            variant: ButtonVariant::Outline,
                            class: "h-9 px-4 py-2",
                            Download { class: "w-4 h-4 mr-2" }
                            "Download CV"
                        }
                    }
                }
            }
        }
    }
}
