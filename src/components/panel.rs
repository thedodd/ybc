use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML content of this panel's heading; it is automatically wrapped in a `p.panel-heading`.
    #[prop_or_default]
    pub heading: Html,
}

/// A composable panel, for compact controls.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let class = classes!("panel", &props.classes);
    html! {
        <nav {class}>
            <p class="panel-heading">{props.heading.clone()}</p>
            {props.children.clone()}
        </nav>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
}

/// A container for the navigation tabs of a panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(PanelTabs)]
pub fn panel_tabs(props: &PanelTabsProps) -> Html {
    html! { <p class="panel-tabs">{props.children.clone()}</p> }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelBlockProps {
    #[prop_or_default]
    pub children: Children,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// Make this element the active / highlighted element.
    #[prop_or_default]
    pub active: bool,
    /// The click handler for this element.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// An individual element of the panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(PanelBlock)]
pub fn panel_block(props: &PanelBlockProps) -> Html {
    let class = classes!("panel-block", props.active.then(|| "is-active"));
    html! {
        <@{props.tag.clone()} {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </@>
    }
}
