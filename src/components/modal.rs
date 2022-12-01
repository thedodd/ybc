use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use yew::prelude::*;

use yew_agent::{use_bridge, HandlerId, Public, UseBridgeHandle, Worker, WorkerLink};

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
    pub classes: Classes,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let is_active = use_state(|| false);

    let mut class = Classes::from("modal");

    class.push(props.classes.clone());

    let (opencb, closecb) = if *is_active {
        class.push("is-active");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |_| is_active.set(false)))
    } else {
        let is_active = is_active.clone();

        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    };

    {
        let id = props.id.clone();

        let _bridge: UseBridgeHandle<ModalCloser> = use_bridge(move |response: ModalCloseMsg| {
            if response.0 == id {
                is_active.set(false);
            } else {
            }
        });
    }

    html! {
        <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-content">
                {props.children.clone()}
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
        </>
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
    pub classes: Classes,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(ModalCard)]
pub fn modal_card(props: &ModalCardProps) -> Html {
    let is_active = use_state(|| false);

    let mut class = Classes::from("modal");
    class.push(props.classes.clone());

    let (opencb, closecb) = if *is_active {
        class.push("is-active");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |_| is_active.set(false)))
    } else {
        let is_active = is_active.clone();

        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    };

    {
        let id = props.id.clone();

        let _bridge: UseBridgeHandle<ModalCloser> = use_bridge(move |response: ModalCloseMsg| {
            if response.0 == id {
                is_active.set(false);
            } else {
            }
        });
    }

    html! {
    <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{props.title.clone()}</p>
                    <button class="delete" aria-label="close" onclick={closecb.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {props.body.clone()}
                </section>
                <footer class="modal-card-foot">
                    {props.footer.clone()}
                </footer>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
    </>
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A request to close a modal instance by ID.
///
/// The ID provided in this message must match the ID of the modal which is to be closed, else
/// the message will be ignored.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    link: WorkerLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for ModalCloser {
    type Input = ModalCloseMsg;
    type Message = ();
    // The agent receives requests to close modals by ID.
    type Output = ModalCloseMsg;
    type Reach = Public<ModalCloser>;

    // The agent forwards the input to all registered modals.

    fn create(link: WorkerLink<Self>) -> Self {
        Self { link, subscribers: HashSet::new() }
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
