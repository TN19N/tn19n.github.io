mod content;
mod description;
mod header;
mod title;

pub use content::CardContent;
pub use description::CardDescription;
pub use header::CardHeader;
pub use title::CardTitle;

use dioxus::prelude::*;

#[component]
pub fn Card(children: Element, #[props(default = true)] hover: bool) -> Element {
    rsx! {
        div {
            class: "text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm shadow-foreground/30 bg-card border-border transition-all",
            class: if hover { "hover:shadow-lg" },
            {children}
        }
    }
}
