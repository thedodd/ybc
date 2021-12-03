#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The input type of this component.
    #[prop_or_else(|| InputType::Text)]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let class = classes!(
        "input",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.rounded.then(|| "is-rounded"),
        props.loading.then(|| "is-loading"),
        props.r#static.then(|| "is-static"),
    );
    let oninput = props.update.reform(|ev: web_sys::InputEvent| {
        let input: HtmlInputElement = ev.target_dyn_into().expect_throw("event target should be an input");
        input.value()
    });
    html! {
        <input
            name={props.name.clone()}
            value={props.value.clone()}
            {oninput}
            {class}
            type={props.r#type.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
pub enum InputType {
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "tel")]
    Tel,
}
