use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// A versatile delete cross.
///
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
#[function_component(Delete)]
pub fn delete(props: &DeleteProps) -> Html {
    let class = classes!("delete", props.classes.clone());
    html! {
        <@{props.tag.clone()} {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </@>
    }
}
