use crate::{
    common::components::ThemeToggle,
    features::home::components::{
        BLOG_SECTION_ID, CONTACT_SECTION_ID, EXPERIENCES_SECTION_ID, HERO_SECTION_ID,
        PROJECTS_SECTION_ID,
    },
    router::AppRouter,
};
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);

    rsx! {
        nav { class: "flex items-center justify-between px-4 py-4 bg-card/50 backdrop-blur-sm gap-6",
            Link {
                to: AppRouter::Home {
                    anchor: "".into(),
                },
                h1 { class: "text-2xl font-bold text-primary", "TN19N" }
            }
            div { class: "hidden md:flex items-center gap-6",
                for (section_id , name) in [
                    (HERO_SECTION_ID, "About"),
                    (BLOG_SECTION_ID, "Blog"),
                    (PROJECTS_SECTION_ID, "Projects"),
                    (EXPERIENCES_SECTION_ID, "Experiences"),
                    (CONTACT_SECTION_ID, "Contact"),
                ]
                {
                    Link {
                        to: AppRouter::Home {
                            anchor: section_id.to_string(),
                        },
                        class: "text-foreground hover:text-primary",
                        "{name}"
                    }
                }
                ThemeToggle {
                }
            }
            div { class: "md:hidden",
                button {
                    "type": "button",
                    "aria-label": "Open mobile menu",
                    onclick: move |_| is_mobile_menu_open.toggle(),
                    svg {
                        "xmlns": "http://www.w3.org/2000/svg",
                        "width": "24",
                        "height": "24",
                        "viewBox": "0 0 24 24",
                        "fill": "none",
                        "stroke": "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path { "d": "M4 6h16M4 12h16M4 18h16" }
                    }
                }
            }
        }
        if is_mobile_menu_open() {
            div {
                class: "fixed inset-0 z-50 bg-background/80 backdrop-blur-sm",
                onclick: move |_| is_mobile_menu_open.set(false),
                div {
                    class: "fixed top-4 right-4 w-full max-w-xs bg-card rounded-lg shadow-lg p-6",
                    div {
                        class: "flex items-center justify-between",
                        h2 { class: "text-lg font-bold", "Menu" }
                        button {
                            "type": "button",
                            "aria-label": "Close mobile menu",
                            onclick: move |_| is_mobile_menu_open.set(false),
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "width": "24",
                                "height": "24",
                                "viewBox": "0 0 24 24",
                                "fill": "none",
                                "stroke": "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                path { "d": "M18 6L6 18M6 6l12 12" }
                            }
                        }
                    }
                    div { class: "mt-6 flex flex-col gap-4",
                        for (section_id , name) in [
                            (HERO_SECTION_ID, "About"),
                            (BLOG_SECTION_ID, "Blog"),
                            (PROJECTS_SECTION_ID, "Projects"),
                            (EXPERIENCES_SECTION_ID, "Experiences"),
                            (CONTACT_SECTION_ID, "Contact"),
                        ]
                        {
                            Link {
                                to: AppRouter::Home {
                                    anchor: section_id.to_string(),
                                },
                                class: "text-foreground hover:text-primary",
                                onclick: move |_| is_mobile_menu_open.set(false),
                                "{name}"
                            }
                        }
                        ThemeToggle {}
                    }
                }
            }
        }
    }
}
