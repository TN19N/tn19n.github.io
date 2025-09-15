use crate::router::AppRouter;
use dioxus::prelude::*;

static TAILWIND: Asset = asset!("./assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND }
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/styles/github.min.css" }
        div { class: "min-h-screen bg-background flex flex-col", Router::<AppRouter> {} }
    }
}
