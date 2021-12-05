use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A UI element for repeatable and nestable content.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(Media)]
pub fn media(props: &MediaProps) -> Html {
    let class = classes!("media", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// Elements to be grouped to the left of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(MediaLeft)]
pub fn media_left(props: &MediaLeftProps) -> Html {
    let class = classes!("media-left", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// Elements to be grouped to the right of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(MediaRight)]
pub fn media_right(props: &MediaRightProps) -> Html {
    let class = classes!("media-right", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// Elements to be grouped as the center body of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(MediaContent)]
pub fn media_content(props: &MediaContentProps) -> Html {
    let class = classes!("media-content", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}
