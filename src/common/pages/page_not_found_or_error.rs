use crate::{
    common::{
        components::{
            Button, ButtonVariant, Card, CardContent, CardDescription, CardHeader, CardTitle,
        },
        icons::{arrow_left::ArrowLeft, house::House},
    },
    router::AppRouter,
};
use dioxus::prelude::*;

#[component]
pub fn PageNotFoundOrError(segments: Vec<String>, #[props(default = 404)] status: u32) -> Element {
    let navigate = use_navigator();

    rsx! {
        section { id: "page-not-found",
            div { class: "max-w-2xl mx-auto text-center",
                Card { hover: false,
                    CardHeader {
                        div { class: "flex flex-col items-center justify-center gap-5",
                            span { class: "text-6xl font-bold text-primary", "{status}" }
                            if status == 404 {
                                CardTitle {
                                    p { class: "text-3xl font-bold mb-2", "Page Not Found" }
                                }
                                CardDescription {
                                    "Oops! The page you're looking for seems to have wandered off into the digital void."
                                }
                            } else {
                                CardTitle {
                                    p { class: "text-3xl font-bold mb-2", "Internal Error" }
                                }
                                CardDescription { "Oops! Something went wrong!" }
                            }
                        }
                    }
                    CardContent {
                        div { class: "space-y-6",
                            div { class: "flex flex-col sm:flex-row gap-4 justify-center",
                                Link {
                                    to: AppRouter::Home {
                                        anchor: "".into(),
                                    },
                                    Button { class: "h-9 px-4 py-2",
                                        House { class: "w-4 h-4 mr-2" }
                                        "Go Home"
                                    }
                                }
                                button {
                                    onclick: move |_| {
                                        navigate.go_back();
                                    },
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        class: "h-9 px-4 py-2",
                                        ArrowLeft { class: "w-4 h-4 mr-2" }
                                        "Go Back"
                                    }
                                }
                            }
                            div { class: "pt-4 border-t border-border",
                                p { class: "text-sm text-muted-foreground",
                                    "If you believe this is an error, feel free to "
                                    a {
                                        class: "text-primary hover:underline",
                                        href: "/#contact",
                                        "contact me"
                                    }
                                    " and I'll investigate the issue."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
