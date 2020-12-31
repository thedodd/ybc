use std::collections::HashSet;

use yew::prelude::*;
use yew::worker::*;
use yewtil::NeqAssign;

/// Modal actions.
pub enum ModalMsg {
    Open,
    Close,
    CloseFromAgent(ModalCloseMsg),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
pub struct Modal {
    props: ModalProps,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    subscription: Box<dyn Bridge<ModalCloser>>,
    is_active: bool,
}

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(ModalMsg::CloseFromAgent);
        let subscription = ModalCloser::bridge(callback);
        Self {
            props,
            link,
            subscription,
            is_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModalMsg::Close => {
                self.is_active = false;
            }
            ModalMsg::Open => {
                self.is_active = true;
            }
            ModalMsg::CloseFromAgent(id) => {
                if id.0 == self.props.id {
                    self.is_active = false;
                } else {
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("modal");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let (opencb, closecb) = if self.is_active {
            classes.push("is-active");
            (Callback::noop(), self.link.callback(|_| ModalMsg::Close))
        } else {
            (self.link.callback(|_| ModalMsg::Open), Callback::noop())
        };
        html! {
            <>
            <div onclick=opencb>
                {self.props.trigger.clone()}
            </div>
            <div id=self.props.id.clone() class=classes>
                <div class="modal-background" onclick=closecb.clone()></div>
                <div class="modal-content">
                    {self.props.children.clone()}
                </div>
                <button class="modal-close is-large" aria-label="close" onclick=closecb></button>
            </div>
            </>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The title of this modal.
    pub title: String,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `modal_title` prop.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
pub struct ModalCard {
    props: ModalCardProps,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    subscription: Box<dyn Bridge<ModalCloser>>,
    is_active: bool,
}

impl Component for ModalCard {
    type Message = ModalMsg;
    type Properties = ModalCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(ModalMsg::CloseFromAgent);
        let subscription = ModalCloser::bridge(callback);
        Self {
            props,
            link,
            subscription,
            is_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModalMsg::Close => {
                self.is_active = false;
            }
            ModalMsg::Open => {
                self.is_active = true;
            }
            ModalMsg::CloseFromAgent(id) => {
                if id.0 == self.props.id {
                    self.is_active = false;
                } else {
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("modal");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let (opencb, closecb) = if self.is_active {
            classes.push("is-active");
            (Callback::noop(), self.link.callback(|_| ModalMsg::Close))
        } else {
            (self.link.callback(|_| ModalMsg::Open), Callback::noop())
        };
        html! {
            <>
            <div onclick=opencb>
                {self.props.trigger.clone()}
            </div>
            <div id=self.props.id.clone() class=classes>
                <div class="modal-background" onclick=closecb.clone()></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{self.props.title.clone()}</p>
                        <button class="delete" aria-label="close" onclick=closecb.clone()></button>
                    </header>
                    <section class="modal-card-body">
                        {self.props.body.clone()}
                    </section>
                    <footer class="modal-card-foot">
                        {self.props.footer.clone()}
                    </footer>
                </div>
                <button class="modal-close is-large" aria-label="close" onclick=closecb.clone()></button>
            </div>
            </>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A request to close a modal instance by ID.
///
/// The ID provided in this message must match the ID of the modal which is to be closed, else
/// the message will be ignored.
#[derive(Clone, Debug)]
pub struct ModalCloseMsg(pub String);

/// An agent used for being able to close `Modal` & `ModalCard` instances by ID.
///
/// If custom modal closing functionality is need for your modal instance, the following
/// pattern is recommended.
///
/// First, in your component which is using this modal, configure a `ModalCloser` dispatcher.
/// ```rust
/// use yew::agent::Dispatcher;
/// use yew::prelude::*;
/// // .. snip ..
/// fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
///     let bridge = ModalCloser::dispatcher();
///     Self { link, props, bridge }
/// }
/// ```
///
/// Next, in your component's `view` method, setup a callback to handle your component's close
/// event. ```rust
/// let closer = self.link.callback(|_| ModalCloseMsg("modal-0".into()));
/// // ... snip ...
/// <ModalCard
///     id="modal-0"
///     // ... snip ...
///     footer=html!{
///         <Button onclick=Some(closer)>{"Close"}</Button>
///     }
/// />
/// ```
///
/// Finally, in your component's `update` method, send the `ModalCloseMsg` over to the agent which
/// will forward the message to the modal to cause it to close.
/// ```rust
/// fn update(&mut self, msg: Self::Message) -> ShouldRender {
///     self.bridge.send(msg);
///     true
/// }
/// ```
///
/// This pattern allows you to communicate with a modal by its given ID, allowing
/// you to close the modal from anywhere in your application.
pub struct ModalCloser {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ModalCloser {
    type Input = ModalCloseMsg;
    type Message = ();
    // The agent receives requests to close modals by ID.
    type Output = ModalCloseMsg;
    type Reach = Context<Self>;

    // The agent forwards the input to all registered modals.

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        for cmp in self.subscribers.iter() {
            self.link.respond(*cmp, msg.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
