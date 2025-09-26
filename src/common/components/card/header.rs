use dioxus::prelude::*;

#[component]
pub fn CardHeader(children: Element) -> Element {
    rsx! {
        div { class: "grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6",
            {children}
        }
    }
}
