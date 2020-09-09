use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub heading: Html,
}

pub struct Panel {
    props: PanelProps,
}

impl Component for Panel {
    type Message = ();
    type Properties = PanelProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
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
        html!{
            <nav class=classes>
                <p class="panel-heading">{self.props.heading.clone()}</p>
                {self.props.children.clone()}
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct PanelTabs {
    props: PanelTabsProps,
}

impl Component for PanelTabs {
    type Message = ();
    type Properties = PanelTabsProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html!{
            <p class="panel-tabs">
                {self.props.children.clone()}
            </p>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelBlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    #[prop_or_default]
    pub active: bool,
}

pub struct PanelBlock {
    props: PanelBlockProps,
}

impl Component for PanelBlock {
    type Message = ();
    type Properties = PanelBlockProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
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
        html!{
            <@{tag} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}
