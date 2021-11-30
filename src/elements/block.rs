use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bulma’s most basic spacer block
///
/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
#[function_component(Block)]
pub fn block(props: &BlockProps) -> Html {
    let class = classes!("block", props.classes.clone());
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}
