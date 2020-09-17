use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// A simple responsive footer which can include anything.
///
/// [https://bulma.io/documentation/layout/footer/](https://bulma.io/documentation/layout/footer/)
pub struct Footer {
    props: FooterProps,
}

impl Component for Footer {
    type Message = ();
    type Properties = FooterProps;

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
        let mut classes = Classes::from("footer");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html! {
            <footer class=classes>
                {self.props.children.clone()}
            </footer>
        }
    }
}
