use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "nav".into())]
    pub tag: String,
}

/// A multi-purpose horizontal level, which can contain almost any other element.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
#[function_component(Level)]
pub fn level(props: &LevelProps) -> Html {
    let class = classes!("level", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the left of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
#[function_component(LevelLeft)]
pub fn level_left(props: &LevelLeftProps) -> Html {
    let class = classes!("level-left", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the right of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
#[function_component(LevelRight)]
pub fn level_right(props: &LevelRightProps) -> Html {
    let class = classes!("level-right", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// An individual element of a level container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
#[function_component(LevelItem)]
pub fn level_item(props: &LevelItemProps) -> Html {
    let class = classes!("level-item", &props.classes);
    html! {
        <@{props.tag.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}
