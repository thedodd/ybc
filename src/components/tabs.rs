use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Add a more classic style with borders to this component.
    #[prop_or_default]
    pub boxed: bool,
    /// Add the "radio button" style to the elements of this component.
    #[prop_or_default]
    pub toggle: bool,
    /// Make the tab elements of this component rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this component fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
}

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
///
/// For integration with Yew Router, it is recommended that the `RouterButton` or `RouterAnchor`
/// components be used as the individual tab elements for this component.
pub struct Tabs {
    props: TabsProps,
}

impl Component for Tabs {
    type Message = ();
    type Properties = TabsProps;

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
        let mut classes = Classes::from("tabs");
        if let Some(extra) = &self.props.classes {
            classes.push(extra);
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.boxed {
            classes.push("is-boxed");
        }
        if self.props.toggle {
            classes.push("is-toggle");
        }
        if self.props.rounded {
            classes.push("is-rounded");
        }
        if self.props.fullwidth {
            classes.push("is-fullwidth");
        }
        html! {
            <div class=classes>
                <ul>
                    {self.props.children.clone()}
                </ul>
            </div>
        }
    }
}
