use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A single component to wrap WYSIWYG generated content, where only HTML tags are available.
///
/// [https://bulma.io/documentation/elements/content/](https://bulma.io/documentation/elements/content/)
#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    html! {
        <@{props.tag.clone()} class={classes!("content", &props.classes)}>
            {props.children.clone()}
        </@>
    }
}
