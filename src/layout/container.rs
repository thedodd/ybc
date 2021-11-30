use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add a `32px` margin to the left and right sides of the container.
    #[prop_or_default]
    pub fluid: bool,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let class = classes!("container", props.classes.clone(), props.fluid.then(|| "is-fluid"));
    html! {
        <div {class}>{props.children.clone()}</div>
    }
}
