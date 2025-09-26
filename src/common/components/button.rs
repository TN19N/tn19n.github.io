use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Outline,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Default)]
    variant: ButtonVariant,
    class: Option<&'static str>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_style = match props.variant {
        ButtonVariant::Default => "text-primary-foreground bg-primary hover:bg-primary/80",
        ButtonVariant::Outline => {
            "text-primary hover:bg-primary hover:text-primary-foreground bg-transparent border-primary border"
        }
    };

    rsx! {
        span {
            class: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all bg-background",
            class: "{variant_style}",
            class: if let Some(class) = props.class { "{class}" },
            {props.children}
        }
    }
}
