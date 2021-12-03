use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A container for any type of icon font.
///
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let class = classes!(
        "icon",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.alignment.as_ref().map(|alignment| alignment.to_string()),
    );
    html! {
        <span {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </span>
    }
}
