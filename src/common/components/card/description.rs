use dioxus::prelude::*;

#[component]
pub fn CardDescription(children: Element) -> Element {
    rsx! {
        div { class: "text-sm flex items-center gap-2 text-muted-foreground", {children} }
    }
}
