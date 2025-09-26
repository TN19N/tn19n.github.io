use crate::{
    common::{layout::AppLayout, pages::page_not_found_or_error::PageNotFoundOrError},
    features::{Blog, Home, Post},
};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum AppRouter {
    #[layout(AppLayout)]
        #[route("/#:anchor")]
        Home { anchor: String },
        #[nest("/blog")]
            #[route("/")]
            Blog {},
            #[route("/post/:post")]
            Post { post: String },
        #[end_nest]
        #[route("/:..segments")]
        PageNotFoundOrError { segments: Vec<String> },
}
