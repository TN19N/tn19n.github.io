## Introduction

In this post, I'll guide you through creating and deploying a simple, modern portfolio and blog using a powerful stack of technologies. We'll build a static site that you can host for free on GitHub Pages, allowing you to share articles and insights while learning the fundamentals of web development with Rust.

## Core Technologies

- **[Rust](https://www.rust-lang.org/):** A language that guarantees memory safety and high performance, making it an excellent choice for building reliable web applications.
- **[Dioxus](https://dioxuslabs.com/):** A modern UI framework for Rust that allows building cross-platform applications for the web, desktop, and mobile. It's inspired by React, which makes it easy to pick up for developers familiar with component-based frameworks.
- **[Moon](https://moonrepo.dev/moon):** A better way to manage monorepos and toolchains. We'll use it to lock down our Rust toolchain and dependency versions, ensuring a consistent development environment.
- **[Tailwind CSS](https://tailwindcss.com/):** A utility-first CSS framework that enables rapid UI development without writing custom CSS.
- **[GitHub Pages](https://pages.github.com/):** For hosting the static website directly from a GitHub repository.
- **[GitHub Actions](https://github.com/features/actions):** To automate the build and deployment process (CI/CD).

## Initial Project Setup

First, let's get our project structure and tooling in place.

### Scaffolding with Dioxus CLI

We'll start by installing the Dioxus CLI, `dx`, to scaffold our new project. For this tutorial, we're using version `0.7.0-rc.0` to take advantage of the latest features.

```bash
cargo install dioxus-cli --version 0.7.0-rc.0
```

With the CLI installed, create a new project:

```bash
dx new portfolio
```

When prompted, make sure to enable the `Router`, `Tailwind CSS`, and `LLMs prompts` options. The last option creates a helpful prompt file that gives context to LLM-based tools and assistants, which is a nice touch from the Dioxus team.

### Pinning Versions with Moon

To ensure our build is reproducible and that all developers (including our future selves) use the same tool versions, we'll use Moon. After installing Moon by following the [official docs](https://moonrepo.dev/docs/install), we'll create a `moon.yml` file in our project root to define our toolchain.

This configuration tells Moon which version of Rust, and other tools to use.

```yaml
# .moon/toolchain.yml
rust:
  version: 1.90.0
  syncToolchainConfig: true
  binstallVersion: 1.15.3
  components:
    - clippy
    - rustfmt
  targets:
    - wasm32-unknown-unknown
  bins:
    - bin: dioxus-cli@0.7.0-rc.0
```

Now, running `moon setup` will automatically install the correct versions of everything we need.

## UI and Components

Dioxus uses a component-based architecture, similar to React. UI is written in `RSX`, a Rust macro that looks almost identical to JSX. This, combined with a reactive state management system using "hooks" like `use_signal`, `use_resource`, and `use_memo`, makes building complex UIs surprisingly straightforward.

### Styling with Tailwind CSS

Thanks to the Dioxus CLI's integration, setting up Tailwind CSS is incredibly simple. In your `portfolio/src/main.rs`, you just need to link the stylesheet:

```rust
// portfolio/src/main.rs
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    rsx! {
        // ... other links
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
```

The `dioxus-cli` automatically processes the `tailwind.css` file in your assets directory during the build, applying all the utility classes you use in your RSX markup and generating a final, optimized CSS file.

### Creating a Reusable Component

Let's look at a simple `Badge` component from this project to see how it all comes together.

```rust
// src/common/components/badge.rs
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BadgeProps {
    children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        div {
            class: "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium",
            {props.children}
        }
    }
}
```

This is a basic, reusable component that accepts `children`. We can use it in any other component like this:

```rust
rsx! {
    Badge { "Hello World" }
}
```

## App Structure and Routing

A multi-page application needs a router. In Dioxus, this is handled elegantly using a combination of a `Routable` enum and a `Router` component.

```rust
// src/router.rs
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum AppRouter {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/blog/:post")]
    Post { post: String },
    // ... other routes
}
```

The `AppRouter` enum defines all the possible routes in the application. The `Router` component, placed in your main `App` component, is responsible for rendering the correct page component based on the current URL.

## Rendering Blog Posts from Markdown

To create a blog, we need a way to convert Markdown files into HTML. This project uses the excellent `comrak` crate, which is a fast and feature-rich Markdown parser.

Here's a simplified look at how a blog post is rendered:

```rust
// src/features/blog/pages/post.rs
use comrak::{markdown_to_html, ComrakOptions};
use dioxus::prelude::*;

#[component]
pub fn Post(post_name: String) -> Element {
    let post_content = use_resource(move || async move {
        // In a real app, you'd fetch your markdown file content here
        let markdown = fetch_markdown_from_file(&post_name).await;
        markdown_to_html(&markdown, &ComrakOptions::default())
    });

    rsx! {
        match post_content.read().as_ref() {
            Some(html) => rsx! {
                article {
                    dangerous_inner_html: "{html}",
                }
            },
            None => rsx! { "Loading..." }
        }
    }
}
```

We use a `use_resource` hook to asynchronously fetch the Markdown content. Then, `comrak::markdown_to_html` converts it to an HTML string, which is rendered using `dangerous_inner_html`. The project also uses a `Syntect` plugin with `comrak` to add syntax highlighting to code blocks automatically.

## Automated Deployment with GitHub Actions

The final piece of the puzzle is automating deployment. We want to automatically build and deploy our site to GitHub Pages every time we push to the `main` branch.

The `.github/workflows/cd.yml` file defines this process:

1.  **Checkout:** It checks out the repository's code.
2.  **Setup Toolchain:** It uses `moonrepo/setup-toolchain` to install the Rust and Moon versions defined in our `moon.yml`.
3.  **Build:** It runs `moon portfolio:build/release` to build the application in release mode. This creates the static assets (HTML, CSS, JS, Wasm) in the `target/dx/portfolio/release/web/public` directory.
4.  **Upload Artifact:** It uploads the built `public` directory as a GitHub Pages artifact.
5.  **Deploy:** A separate `deploy` job takes this artifact and deploys it to your GitHub Pages site.

With this workflow in place, your portfolio is automatically updated on every push to `main`.

## Conclusion

We've covered a lot of ground: setting up a reproducible development environment with Moon, building a component-based UI with Dioxus and Tailwind CSS, handling routing, rendering Markdown for a blog, and automating deployment with GitHub Actions.

This stack provides a robust foundation for building high-performance, maintainable web applications in Rust. You now have a live portfolio and blog to share your work and ideas with the world. Happy coding!