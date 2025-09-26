use crate::{common::providers::theme::ThemeProvider, router::AppRouter};
use dioxus::{prelude::*, router::RouterConfig};

static TAILWIND: Asset = asset!("./assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND }
        ThemeProvider {
            Router::<AppRouter> { config: RouterConfig::default }
        }
    }
}
