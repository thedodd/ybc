use derive_more::Display;
use yew::events::{Event, FocusEvent, MouseEvent};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The size for all buttons within this group.
    #[prop_or_default]
    pub size: Option<ButtonGroupSize>,
}

/// A container for a group of buttons.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct Buttons {
    props: ButtonsProps,
}

impl Component for Buttons {
    type Message = ();
    type Properties = ButtonsProps;

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
        let mut classes = Classes::from("buttons");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

/// The 3 sizes available for a button group.
///
/// https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "are-{}")]
pub enum ButtonGroupSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
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
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A button element.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct Button {
    props: ButtonProps,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

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
        html! {
            <button class=classes onclick=self.props.onclick.clone() disabled=self.props.disabled>
                {self.props.children.clone()}
            </button>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use yew_router::components::{RouterAnchor, RouterButton as RouterBtn};
    use yew_router::{RouterState, Switch};

    #[derive(Clone, Properties, PartialEq)]
    pub struct ButtonRouterProps<SW: Switch + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: SW,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Option<String>,
        /// Render a loading spinner within this component.
        #[prop_or_default]
        pub loading: bool,
        /// Make this component static.
        #[prop_or_default]
        pub r#static: bool,
        /// Disable this component.
        #[prop_or_default]
        pub disabled: bool,
    }

    /// A Yew Router button element with Bulma styling.
    pub struct ButtonRouter<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState = ()> {
        props: ButtonRouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for ButtonRouter<SW, STATE> {
        type Message = ();
        type Properties = ButtonRouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self {
                props,
                marker: std::marker::PhantomData,
            }
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
            html! {
                <RouterBtn<SW, STATE>
                    route=self.props.route.clone()
                    disabled=self.props.disabled
                    classes=classes.to_string()
                    children=self.props.children.clone()
                />
            }
        }
    }

    /// A Yew Router anchor button element with Bulma styling.
    pub struct ButtonAnchorRouter<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState = ()> {
        props: ButtonRouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for ButtonAnchorRouter<SW, STATE> {
        type Message = ();
        type Properties = ButtonRouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self {
                props,
                marker: std::marker::PhantomData,
            }
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
            html! {
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

#[cfg(feature = "router")]
pub use router::{ButtonAnchorRouter, ButtonRouter, ButtonRouterProps};

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonAnchorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The `href` attribute value to use for this component.
    #[prop_or_default]
    pub href: String,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// An anchor element styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonAnchor {
    props: ButtonAnchorProps,
}

impl Component for ButtonAnchor {
    type Message = ();
    type Properties = ButtonAnchorProps;

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
        html! {
            <a
                class=classes
                onclick=self.props.onclick.clone()
                href=self.props.href.clone()
                rel=self.props.rel.clone().unwrap_or_default()
                target=self.props.target.clone().unwrap_or_default()
                disabled=self.props.disabled
            >
                {self.props.children.clone()}
            </a>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputSubmitProps {
    #[prop_or_default]
    pub classes: Option<String>,
    /// The submit handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onsubmit: Callback<FocusEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="submit"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonInputSubmit {
    props: ButtonInputSubmitProps,
}

impl Component for ButtonInputSubmit {
    type Message = ();
    type Properties = ButtonInputSubmitProps;

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
        html! {
            <input type="submit" class=classes onsubmit=self.props.onsubmit.clone() disabled=self.props.disabled/>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputResetProps {
    #[prop_or_default]
    pub classes: Option<String>,
    /// The reset handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onreset: Callback<Event>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="reset"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonInputReset {
    props: ButtonInputResetProps,
}

impl Component for ButtonInputReset {
    type Message = ();
    type Properties = ButtonInputResetProps;

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
        html! {
            <input type="reset" class=classes onreset=self.props.onreset.clone() disabled=self.props.disabled/>
        }
    }
}
