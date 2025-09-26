use crate::{common::providers::theme::ThemeProvider, router::AppRouter};
use dioxus::{prelude::*, router::RouterConfig};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND: Asset = asset!("./assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND }
        ThemeProvider {
            Router::<AppRouter> { config: RouterConfig::default }
        }
    }
}
