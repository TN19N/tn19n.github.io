use crate::router::AppRouter;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        header { class: "border-b border-border bg-card/50 backdrop-blur-sm sticky top-0 z-50",
            div { class: "container mx-auto px-4 py-4",
                nav { class: "flex items-center justify-between",
                    h1 { class: "text-2xl font-bold text-primary", "TN19N" }
                    div { class: "flex items-center gap-6",
                        a {
                            href: "/#about",
                            class: "text-foreground hover:text-primary transition-colors",
                            "About"
                        }
                        a {
                            href: "/#blog",
                            class: "text-foreground hover:text-primary transition-colors",
                            "Blog"
                        }
                        a {
                            href: "/#projects",
                            class: "text-foreground hover:text-primary transition-colors",
                            "Projects"
                        }
                        a {
                            href: "/#experiences",
                            class: "text-foreground hover:text-primary transition-colors",
                            "Experience"
                        }
                        a {
                            href: "/#contact",
                            class: "text-foreground hover:text-primary transition-colors",
                            "Contact"
                        }
                    }
                }
            }
        }
        Outlet::<AppRouter> {}
    }
}
