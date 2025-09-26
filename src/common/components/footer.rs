use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "py-8 border-t border-border bg-card/50",
            div { class: "container mx-auto px-4 text-center",
                p { class: "text-muted-foreground",
                    "© 2025 TN19N. Built with "
                    em {
                        a {
                            href: "https://dioxuslabs.com/",
                            class: "text-primary",
                            target: "__blank",
                            "Dioxus"
                        }
                    }
                    "."
                }
            }
        }
    }
}
