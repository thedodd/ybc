use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "span".into())]
    pub tag: String,
    /// The click handler for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Make this tag rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Turn this tag into a delete button.
    #[prop_or_default]
    pub delete: bool,
    /// The size for this component.
    #[prop_or_default]
    pub size: Option<Size>,
}

/// A small tag label to insert anywhere.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let class = classes!(
        "tag",
        props.classes.clone(),
        props.rounded.then_some("is-rounded"),
        props.delete.then_some("is-delete"),
        props.size.as_ref().map(|size| size.to_string()),
    );
    html! {
        <@{props.tag.clone()} {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Attach two tags together; this requires that this component wraps two `Tag` components.
    #[prop_or_default]
    pub has_addons: bool,
}

/// A container for a list of tags.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tags)]
pub fn tags(props: &TagsProps) -> Html {
    let class = classes!("tags", props.classes.clone(), props.has_addons.then_some("has-addons"));
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}
