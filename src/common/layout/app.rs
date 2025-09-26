use crate::{
    common::{
        components::{Footer, NavBar},
        pages::page_not_found_or_error::PageNotFoundOrError,
    },
    router::AppRouter,
};
use dioxus::{logger::tracing, prelude::*};

#[component]
pub fn AppLayout() -> Element {
    let route: AppRouter = use_route();

    let body_layout = match route {
        AppRouter::PageNotFoundOrError { .. } => "flex flex-1 items-center justify-center",
        _ => "flex flex-col flex-1",
    };

    rsx! {
        div { class: "min-h-screen bg-background flex flex-col",
            div { class: "sticky top-0", NavBar {} }
            ErrorBoundary {
                handle_error: |error: ErrorContext| {
                    tracing::error!("Internal Error: {:?}", error);

                    rsx! {
                        div { class: "flex flex-1 items-center justify-center",
                            PageNotFoundOrError { segments: vec![], status: 500 }
                        }
                    }
                },
                div { class: body_layout, Outlet::<AppRouter> {} }
            }
            Footer {}
        }
    }
}
