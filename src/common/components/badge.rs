use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BadgeVariant {
    Default,
    Outline,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BadgeProps {
    class: Option<&'static str>,
    #[props(default = BadgeVariant::Default)]
    variant: BadgeVariant,
    children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        div {
            class: if props.variant == BadgeVariant::Default { "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden border-transparent bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90" } else if props.variant == BadgeVariant::Outline { "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 [&>svg]:size-3 gap-1 [&>svg]:pointer-events-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 aria-invalid:border-destructive transition-[color,box-shadow] overflow-hidden [a&]:hover:bg-accent [a&]:hover:text-accent-foreground" },
            class: if let Some(class) = props.class { "{class}" },
            {props.children}
        }
    }
}
