// use yew::prelude::*;
// use yew::worker::*;

// // pub trait FormAgent {
// //     /// The concrete type used for events coming from managed form components.
// //     ///
// //     /// This type should implement `From` for all form component event types of this library used
// //     /// by the managed form, which must be implemented by the user. This ensures that the form
// //     /// event dispactchers from this library can properly send update events to an instance of
// //     /// `FormAgent` type.
// //     type Input: Clone;

// //     fn register_manager(&mut self, id: HandlerId);
// // }



// // pub struct FormBridge<T> {
// //     bridge: Box<dyn Bridge<FormAgent<T>>>,
// // }

// // impl<T> FormBridge<T> {
// //     pub fn new(callback: Callback<T>) -> Self {

// //     }
// // }



// /// A type which bridges the data coming from form components into a form manager.
// pub struct FormAgent<T> {
//     link: AgentLink<Self>,

//     /// The handler ID of the form manager.
//     ///
//     /// The form manager should always be the component which is responsible for receiving,
//     /// validating, and propagating form state to all of the components of the form it is managing.
//     form_manager: Option<HandlerId>,
// }

// impl Agent for ModalCloser {
//     type Reach = Private<Self>;
//     type Message = ();
//     type Input = ModalCloseMsg; // The agent receives requests to close modals by ID.
//     type Output = ModalCloseMsg; // The agent forwards the input to all registered modals.

//     fn create(link: AgentLink<Self>) -> Self {
//         Self{link, subscribers: HashSet::new()}
//     }

//     fn update(&mut self, _: Self::Message) {}

//     fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
//         for cmp in self.subscribers.iter() {
//             self.link.respond(*cmp, msg.clone());
//         }
//     }

//     fn connected(&mut self, id: HandlerId) {
//         self.subscribers.insert(id);
//     }

//     fn disconnected(&mut self, id: HandlerId) {
//         self.subscribers.remove(&id);
//     }
// }
