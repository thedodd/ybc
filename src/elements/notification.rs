use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

pub struct Notification {
    props: NotificationProps,
}

impl Component for Notification {
    type Message = ();
    type Properties = NotificationProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("notification");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}
