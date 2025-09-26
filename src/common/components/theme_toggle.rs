use crate::common::{
    components::{Button, ButtonVariant},
    providers::theme::Theme,
};
use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_context::<Signal<Theme>>();
    let mut show_theme_menu = use_signal(|| false);

    rsx! {
        div {
            button {
                class: "relative",
                onclick: move |ev| {
                    ev.stop_propagation();
                    show_theme_menu.toggle();
                },
                Button {
                    class: "size-9 hover:bg-primary",
                    variant: ButtonVariant::Outline,
                    {theme().icon("w-4 h-4")}
                }
                span { class: "sr-only", "Theme Menu" }
                if show_theme_menu() {
                    div {
                        class: "absolute right-0 mt-2 origin-top-right rounded-md bg-popover text-popover-foreground",
                        role: "menu",
                        aria_orientation: "vertical",
                        ul { class: "text-sm",
                            for current in Theme::all() {
                                li {
                                    button {
                                        class: "flex w-full items-center px-3 py-2 gap-2 rounded-sm hover:bg-accent hover:text-accent-foreground",
                                        onclick: move |ev| {
                                            ev.stop_propagation();
                                            *theme.write() = current.clone();
                                            show_theme_menu.toggle();
                                        },
                                        {current.icon("mr-2 w-4 h-4")}
                                        "{capitalize(&current.to_string())}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
    }
}
