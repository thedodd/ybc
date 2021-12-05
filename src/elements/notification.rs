use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bold notification blocks, to alert your users of something.
///
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let class = classes!("notification", &props.classes);
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}
