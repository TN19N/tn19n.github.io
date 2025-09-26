use crate::common::{
    components::{Button, ButtonVariant},
    icons::{linkedin::Linkedin, mail::Mail},
};
use dioxus::prelude::*;
use std::collections::HashMap;

pub const CONTACT_SECTION_ID: &str = "contact";

#[component]
pub fn Contact(elements: Signal<HashMap<&'static str, Event<MountedData>>>) -> Element {
    rsx! {
        section {
            id: CONTACT_SECTION_ID,
            class: "py-20 bg-muted/30",
            onmounted: move |element| {
                elements.write().insert(CONTACT_SECTION_ID, element);
            },
            div { class: "container mx-auto px-4",
                h3 { class: "text-3xl font-bold text-center mb-12 text-card-foreground",
                    "Get In Touch"
                }
                div { class: "flex justify-center gap-6 mt-8",
                    for (link , icon) in [
                        ("https://www.linkedin.com/in/mustapha-annouaoui", rsx! {
                            Linkedin { class: "w-5 h-5" }
                        }),
                        ("mailto:mostafaanawawi@gmail.com", rsx! {
                            Mail { class: "w-5 h-5" }
                        }),
                    ]
                    {
                        a { href: link, target: "_blank",
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "size-9",
                                {icon}
                            }
                        }
                    }
                }
            }
        }
    }
}
