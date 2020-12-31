#![allow(clippy::redundant_closure_call)]

use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub id: String,
    /// The HTML content of this panel's heading; it is automatically wrapped in a
    /// `p.panel-heading`.
    #[prop_or_default]
    pub heading: Html,
}

/// A composable panel, for compact controls.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct Panel {
    props: PanelProps,
}

impl Component for Panel {
    type Message = ();
    type Properties = PanelProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("panel");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let id = &self.props.id;
        html! {
            <nav class=classes id=id>
                <p class="panel-heading">{self.props.heading.clone()}</p>
                {self.props.children.clone()}
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
}

/// A container for the navigation tabs of a panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct PanelTabs {
    props: PanelTabsProps,
}

impl Component for PanelTabs {
    type Message = ();
    type Properties = PanelTabsProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <p class="panel-tabs">
                {self.props.children.clone()}
            </p>
        }
    }
}

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
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// An individual element of the panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct PanelBlock {
    props: PanelBlockProps,
}

impl Component for PanelBlock {
    type Message = ();
    type Properties = PanelBlockProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("panel-block");
        if self.props.active {
            classes.push("is-active");
        }
        let tag = self.props.tag.clone();
        html! {
            <@{tag} class=classes onclick=self.props.onclick.clone()>
                {self.props.children.clone()}
            </@>
        }
    }
}
