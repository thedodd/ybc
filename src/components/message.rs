use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Colored message blocks, to emphasize part of your page.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    html! {
        <article class={classes!("message", props.classes.clone())}>
            {props.children.clone()}
        </article>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// An optional message header that can hold a title and a delete element.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(MessageHeader)]
pub fn message_header(props: &MessageHeaderProps) -> Html {
    html! {
        <div class={classes!("message-header", props.classes.clone())}>
            {props.children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for the body of a message.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(MessageBody)]
pub fn message_body(props: &MessageBodyProps) -> Html {
    html! {
        <div class={classes!("message-body", props.classes.clone())}>
            {props.children.clone()}
        </div>
    }
}
