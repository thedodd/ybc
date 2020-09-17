use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// Add a `32px` margin to the left and right sides of the container.
    #[prop_or_default]
    pub fluid: bool,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
pub struct Container {
    props: ContainerProps,
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

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
        let mut classes = Classes::from("container");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.fluid {
            classes.push("is-fluid");
        }
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}
