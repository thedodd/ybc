use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
#[function_component(Box)]
pub fn r#box(props: &BoxProps) -> Html {
    html! {
        <div class={classes!("box", &props.classes)}>
            {props.children.clone()}
        </div>
    }
}
