use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// Colored message blocks, to emphasize part of your page.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
pub struct Message {
    props: MessageProps,
}

impl Component for Message {
    type Message = ();
    type Properties = MessageProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("message");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <article class=classes>
                {self.props.children.clone()}
            </article>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// An optional message header that can hold a title and a delete element.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
pub struct MessageHeader {
    props: MessageHeaderProps,
}

impl Component for MessageHeader {
    type Message = ();
    type Properties = MessageHeaderProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("message-header");
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

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// A container for the body of a message.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
pub struct MessageBody {
    props: MessageBodyProps,
}

impl Component for MessageBody {
    type Message = ();
    type Properties = MessageBodyProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("message-body");
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
