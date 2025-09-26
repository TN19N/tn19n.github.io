use crate::features::home::components::{Blog, Contact, Experiences, Hero, Projects};
use dioxus::prelude::*;
use std::{collections::HashMap, time::Duration};

#[component]
pub fn Home(anchor: ReadSignal<Option<String>>) -> Element {
    let elements = use_signal::<HashMap<&'static str, Event<MountedData>>>(HashMap::new);

    use_effect(move || {
        if let Some(anchor) = &*anchor.read()
            && let Some(element) = elements.read().get(anchor.as_str()).cloned()
        {
            spawn(async move {
                gloo_timers::future::sleep(Duration::from_millis(25)).await;
                element.scroll_to(ScrollBehavior::Instant).await.unwrap();
            });
        }
    });

    rsx! {
        Hero { elements }
        Blog { elements }
        Projects { elements }
        Experiences { elements }
        Contact { elements }
    }
}
