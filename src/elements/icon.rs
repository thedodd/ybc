use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A container for any type of icon font.
///
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
pub struct Icon {
    props: IconProps,
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

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
        let mut classes = Classes::from("icon");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        html! {
            <span class=classes onclick=self.props.onclick.clone()>
                {self.props.children.clone()}
            </span>
        }
    }
}
