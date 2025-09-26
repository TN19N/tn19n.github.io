use crate::{
    common::{
        components::{Badge, Button, ButtonVariant},
        icons::{arrow_left::ArrowLeft, calendar::Calendar},
        model::{posts::get_post_by_name, posts_metadata::get_post_metadata_by_name},
    },
    router::AppRouter,
};
use comrak::{
    ExtensionOptions, ParseOptions, Plugins, RenderOptions, markdown_to_html_with_plugins,
    plugins::syntect::SyntectAdapter,
};
use dioxus::{core::throw_error, prelude::*};

#[derive(Debug, Clone, PartialEq, Props)]
pub struct PostProps {
    post: ReadOnlySignal<String>,
}

#[component]
pub fn Post(props: PostProps) -> Element {
    let metadata =
        use_resource(move || async move { get_post_metadata_by_name(&props.post.read()).await });
    let post = use_resource(move || async move {
        get_post_by_name(&props.post.read()).await.map(|post| {
            let adapter = SyntectAdapter::new(Some("base16-ocean.dark"));
            let mut plugins = Plugins::default();
            plugins.render.codefence_syntax_highlighter = Some(&adapter);

            markdown_to_html_with_plugins(
                &post,
                &comrak::Options {
                    extension: ExtensionOptions::builder()
                        .strikethrough(true)
                        .table(true)
                        .autolink(true)
                        .tasklist(true)
                        .superscript(true)
                        .footnotes(true)
                        .description_lists(true)
                        .alerts(true)
                        .math_code(true)
                        .math_dollars(true)
                        .build(),
                    parse: ParseOptions::builder().build(),
                    render: RenderOptions::builder()
                        .github_pre_lang(true)
                        .full_info_string(true)
                        .unsafe_(true)
                        .build(),
                },
                &plugins,
            )
        })
    })
    .suspend()?;

    rsx! {
        main { class: "flex-1",
            match metadata.suspend()?.read().as_ref() {
                Ok(Some(metadata)) => rsx! {
                    section { class: "py-16 bg-gradient-to-br from-background to-muted",
                        div { class: "container mx-auto px-4 max-w-4xl",
                            div { class: "flex items-center gap-4 mb-6",
                                Link { to: AppRouter::Blog {},
                                    // Post Content
                                    Button { class: "h-9 px-4 py-2", variant: ButtonVariant::Outline,
                                        ArrowLeft { class: "w-4 h-4 mr-2" }
                                        "Back to Blog"
                                    }
                                }
                            }
                            h1 { class: "text-4xl font-bold text-primary mb-4 text-balance", "{metadata.title}" }
                            div { class: "flex items-center gap-6 text-muted-foreground mb-6",
                                div { class: "flex items-center gap-2",
                                    Calendar { class: "w-4 h-4" }
                                    "{metadata.date.format(\"%b %d, %Y\")}"
                                }
                            }
                            div { class: "flex flex-wrap gap-2",
                                for tag in metadata.tags.iter() {
                                    Badge { "{tag}" }
                                }
                            }
                        }
                    }
                },
                Ok(None) => {
                    navigator()
                        .push(AppRouter::PageNotFoundOrError {
                            segments: vec!["PageNotFound".into()],
                        });
                    rsx! { "Post metadata not found!" }
                }
                Err(err) => {
                    throw_error(err.clone());
                    rsx! { "Failed to display post meatadata" }
                }
            }
            match post.read().as_ref() {
                Ok(post) => rsx! {
                    article { class: "py-16",
                        div { class: "container mx-auto px-4 max-w-4xl",
                            div { class: "prose prose-lg max-w-none",
                                div { dangerous_inner_html: "{post}" }
                            }
                        }
                    }
                },
                Err(err) => {
                    throw_error(err.clone());
                    rsx! { "Failed to display post meatadata" }
                }
            }
        }
    }
}
