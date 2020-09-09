use derive_more::Display;
use yew::prelude::*;
use yew::events::{Event, FocusEvent, MouseEvent};
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub are: Option<ButtonGroupSize>,
}

pub struct Buttons {
    props: ButtonsProps,
}

impl Component for Buttons {
    type Message = ();
    type Properties = ButtonsProps;

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
        let mut classes = Classes::from("buttons");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(are) = &self.props.are {
            classes.push(&are.to_string());
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

/// The three sizes available for a button group.
///
/// https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="are-{}")]
pub enum ButtonGroupSize {
    #[display(fmt="small")]
    Small,
    #[display(fmt="medium")]
    Medium,
    #[display(fmt="large")]
    Large,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub r#static: bool,
    #[prop_or_default]
    pub disabled: bool,
}

pub struct Button {
    props: ButtonProps,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

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
        let mut classes = Classes::from("button");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.loading {
            classes.push("is-loading")
        }
        if self.props.r#static {
            classes.push("is-static")
        }
        if self.props.disabled {
            classes.push("is-disabled")
        }
        html!{
            <button class=classes onclick?=self.props.onclick.clone()>
                {self.props.children.clone()}
            </button>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature="router")]
mod router {
    use super::*;
    use yew_router::{RouterState, Switch};
    use yew_router::components::{RouterAnchor, RouterButton as RouterBtn};

    #[derive(Clone, Properties, PartialEq)]
    pub struct RouterProps<SW: Switch + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: SW,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Option<String>,
        #[prop_or_default]
        pub loading: bool,
        #[prop_or_default]
        pub r#static: bool,
        #[prop_or_default]
        pub disabled: bool,
    }

    pub struct RouterButton<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState=()> {
        props: RouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for RouterButton<SW, STATE> {
        type Message = ();
        type Properties = RouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self{props, marker: std::marker::PhantomData}
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.props.neq_assign(props)
        }

        #[allow(deprecated)]
        fn view(&self) -> Html {
            let mut classes = Classes::from(&self.props.classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if self.props.loading {
                classes.push("is-loading");
            }
            html!{
                <RouterBtn<SW, STATE>
                    route=self.props.route.clone()
                    disabled=self.props.disabled
                    classes=classes.to_string()
                    children=self.props.children.clone()
                />
            }
        }
    }

    pub struct RouterAnchorButton<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState=()> {
        props: RouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for RouterAnchorButton<SW, STATE> {
        type Message = ();
        type Properties = RouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self{props, marker: std::marker::PhantomData}
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.props.neq_assign(props)
        }

        #[allow(deprecated)]
        fn view(&self) -> Html {
            let mut classes = Classes::from(&self.props.classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if self.props.loading {
                classes.push("is-loading");
            }
            html!{
                <RouterAnchor<SW, STATE>
                    route=self.props.route.clone()
                    disabled=self.props.disabled
                    classes=classes.to_string()
                    children=self.props.children.clone()
                />
            }
        }
    }
}

#[cfg(feature="router")]
pub use router::RouterButton;
pub use router::RouterAnchorButton;

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AnchorButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub r#static: bool,
    #[prop_or_default]
    pub disabled: bool,
}

pub struct AnchorButton {
    props: AnchorButtonProps,
}

impl Component for AnchorButton {
    type Message = ();
    type Properties = AnchorButtonProps;

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
        let mut classes = Classes::from("button");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.loading {
            classes.push("is-loading")
        }
        if self.props.r#static {
            classes.push("is-static")
        }
        if self.props.disabled {
            classes.push("is-disabled")
        }
        html!{
            <a class=classes onclick?=self.props.onclick.clone() href?=self.props.href.clone()>
                {self.props.children.clone()}
            </a>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputSubmitButtonProps {
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub onsubmit: Option<Callback<FocusEvent>>,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub r#static: bool,
    #[prop_or_default]
    pub disabled: bool,
}

pub struct InputSubmitButton {
    props: InputSubmitButtonProps,
}

impl Component for InputSubmitButton {
    type Message = ();
    type Properties = InputSubmitButtonProps;

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
        let mut classes = Classes::from("button");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.loading {
            classes.push("is-loading")
        }
        if self.props.r#static {
            classes.push("is-static")
        }
        if self.props.disabled {
            classes.push("is-disabled")
        }
        html!{
            <input type="submit" class=classes onsubmit?=self.props.onsubmit.clone()/>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputResetButtonProps {
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub onreset: Option<Callback<Event>>,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub r#static: bool,
    #[prop_or_default]
    pub disabled: bool,
}

pub struct InputResetButton {
    props: InputResetButtonProps,
}

impl Component for InputResetButton {
    type Message = ();
    type Properties = InputResetButtonProps;

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
        let mut classes = Classes::from("button");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.loading {
            classes.push("is-loading")
        }
        if self.props.r#static {
            classes.push("is-static")
        }
        if self.props.disabled {
            classes.push("is-disabled")
        }
        html!{
            <input type="reset" class=classes onreset?=self.props.onreset.clone()/>
        }
    }
}
