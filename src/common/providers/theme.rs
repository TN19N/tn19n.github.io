use crate::common::icons::{monitor::Monitor, moon::Moon, sun::Sun};
use dioxus::{logger::tracing, prelude::*};
use dioxus_sdk::{
    storage::use_persistent,
    window::theme::{self, use_system_theme},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    System,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dark => "dark",
                Self::Light => "light",
                Self::System => "system",
            }
        )
    }
}

impl Theme {
    pub fn all() -> &'static [Self] {
        &[Self::System, Self::Light, Self::Dark]
    }

    pub fn icon(&self, class: &str) -> Element {
        match self {
            Self::System => rsx! {
                Monitor { class }
            },
            Self::Dark => rsx! {
                Moon { class }
            },
            Self::Light => rsx! {
                Sun { class }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ThemeProviderProps {
    #[props(default = Theme::System)]
    default_theme: Theme,
    children: Element,
}

const THEME_STORAGE_KEY: &str = "theme";

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let theme = use_persistent(THEME_STORAGE_KEY, || props.default_theme);
    let system_theme = use_system_theme();

    provide_context(theme);

    let theme = use_memo(move || {
        let theme = theme();
        match theme {
            Theme::System => match system_theme() {
                Ok(theme) => match theme {
                    theme::Theme::Dark => Theme::Dark.to_string(),
                    theme::Theme::Light => Theme::Light.to_string(),
                },
                Err(theme::ThemeError::Unsupported) => {
                    // in case unsupported theme lets keep using light mode
                    // also to prevent hydration from failing
                    let theme = Theme::Light;
                    tracing::warn!(
                        "Unknown System Theme, default back to {theme:?} theme (it could be because hydration)"
                    );
                    theme.to_string()
                }
                Err(error) => {
                    panic!("Getting System theme is failed: {error:?}");
                }
            },
            theme => theme.to_string(),
        }
    });

    rsx! {
        div { class: "{theme()}", {props.children} }
    }
}
