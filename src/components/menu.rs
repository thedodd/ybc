use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

pub struct Menu {
    props: MenuProps,
}

impl Component for Menu {
    type Message = ();
    type Properties = MenuProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("menu");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <aside class=classes>
                {self.props.children.clone()}
            </aside>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuListProps {
    /// The child `li` elements of this list.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

pub struct MenuList {
    props: MenuListProps,
}

impl Component for MenuList {
    type Message = ();
    type Properties = MenuListProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("menu-list");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <ul class=classes>
                {self.props.children.clone()}
            </ul>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuLabelProps {
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub text: String,
}

pub struct MenuLabel {
    props: MenuLabelProps,
}

impl Component for MenuLabel {
    type Message = ();
    type Properties = MenuLabelProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("menu-label");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <p class=classes>
                {self.props.text.clone()}
            </p>
        }
    }
}
