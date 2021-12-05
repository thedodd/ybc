use derive_more::Display;
use yew::events::{Event, FocusEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size for all buttons within this group.
    #[prop_or_default]
    pub size: Option<ButtonGroupSize>,
}

/// A container for a group of buttons.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProps) -> Html {
    let class = classes!("buttons", &props.classes, props.size.as_ref().map(ToString::to_string));
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
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
    pub classes: Option<Classes>,
    /// The click handler to use for this component.
    #[prop_or_default]
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
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = classes!(
        "button",
        &props.classes,
        props.loading.then(|| "is-loading"),
        props.r#static.then(|| "is-static")
    );
    html! {
        <button {class} onclick={props.onclick.clone()} disabled={props.disabled}>
            {props.children.clone()}
        </button>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use serde::Serialize;
    use yew_router::components::Link;
    use yew_router::Routable;

    #[derive(Clone, Properties, PartialEq)]
    pub struct ButtonRouterProps<R: Routable + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Option<Classes>,
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
    pub struct ButtonRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for ButtonRouter<R, Q> {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let mut classes = Classes::from(&ctx.props().classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if ctx.props().loading {
                classes.push("is-loading");
            }
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    disabled={ctx.props().disabled}
                    {classes}
                    children={ctx.props().children.clone()}
                />
            }
        }
    }

    /// A Yew Router anchor button element with Bulma styling.
    pub struct ButtonAnchorRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for ButtonAnchorRouter<R, Q> {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let mut classes = Classes::from(&ctx.props().classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if ctx.props().loading {
                classes.push("is-loading");
            }
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    disabled={ctx.props().disabled}
                    {classes}
                    children={ctx.props().children.clone()}
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
    pub classes: Option<Classes>,
    /// The `href` attribute value to use for this component.
    #[prop_or_default]
    pub href: String,
    /// The click handler to use for this component.
    #[prop_or_default]
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
#[function_component(ButtonAnchor)]
pub fn button_anchor(props: &ButtonAnchorProps) -> Html {
    let class = classes!(
        "button",
        &props.classes,
        props.loading.then(|| "is-loading"),
        props.r#static.then(|| "is-static")
    );
    html! {
        <a
            {class}
            onclick={props.onclick.clone()}
            href={props.href.clone()}
            rel={props.rel.clone().unwrap_or_default()}
            target={props.target.clone().unwrap_or_default()}
            disabled={props.disabled}
        >
            {props.children.clone()}
        </a>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputSubmitProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The submit handler to use for this component.
    #[prop_or_default]
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
#[function_component(ButtonInputSubmit)]
pub fn button_input_submit(props: &ButtonInputSubmitProps) -> Html {
    let class = classes!(
        "button",
        &props.classes,
        props.loading.then(|| "is-loading"),
        props.r#static.then(|| "is-static"),
    );
    html! {
        <input type="submit" {class} onsubmit={props.onsubmit.clone()} disabled={props.disabled} />
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputResetProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The reset handler to use for this component.
    #[prop_or_default]
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
#[function_component(ButtonInputReset)]
pub fn button_input_reset(props: &ButtonInputResetProps) -> Html {
    let class = classes!(
        "button",
        &props.classes,
        props.loading.then(|| "is-loading"),
        props.r#static.then(|| "is-static"),
    );
    html! {
        <input type="reset" {class} onreset={props.onreset.clone()} disabled={props.disabled} />
    }
}
