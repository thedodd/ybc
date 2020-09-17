#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
pub struct Control {
    props: ControlProps,
}

impl Component for Control {
    type Message = ();
    type Properties = ControlProps;

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
        let mut classes = Classes::from("control");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.expanded {
            classes.push("is-expanded");
        }
        let tag = self.props.tag.clone();
        html! {
            <@{tag} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}
