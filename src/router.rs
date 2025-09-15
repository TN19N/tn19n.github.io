use crate::{
    common::{
        layout::{Footer, NavBar},
        pages::PageNotFound,
    },
    features::{Blog, Home, Post},
};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum AppRouter {
    #[layout(NavBar)]
        #[layout(Footer)]
            #[route("/")]
            Home {},
            #[nest("/blog")]
                #[route("/")]
                Blog {},
                #[route("/:post")]
                Post { post: String },
            #[end_nest] // Nest("/blog")
        #[end_layout] // Layout(Footer)
    #[end_layout] // Layout(NavBar)

    #[route("/..segments")]
    PageNotFound { segments: Vec<String> },
}
