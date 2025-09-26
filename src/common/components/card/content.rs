use dioxus::prelude::*;

#[component]
pub fn CardContent(children: Element) -> Element {
    rsx! {
        div { class: "px-6", {children} }
    }
}
