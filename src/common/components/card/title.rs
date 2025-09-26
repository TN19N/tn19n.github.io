use dioxus::prelude::*;

#[component]
pub fn CardTitle(children: Element) -> Element {
    rsx! {
        div { class: "leading-none font-semibold text-card-foreground flex items-center justify-between",
            {children}
        }
    }
}
