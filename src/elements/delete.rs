#![allow(clippy::redundant_closure_call)]
use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A versatile delete cross.
///
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
pub struct Delete {
    props: DeleteProps,
}

impl Component for Delete {
    type Message = ();
    type Properties = DeleteProps;

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
        let mut classes = Classes::from("delete");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let tag = self.props.tag.clone();
        html! {
            <@{tag} class=classes onclick=self.props.onclick.clone()>
                {self.props.children.clone()}
            </@>
        }
    }
}
