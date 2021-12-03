use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: u32,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed_size: bool,
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

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let class = classes!(
        "textarea",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.loading.then(|| "is-loading"),
        props.r#static.then(|| "is-static"),
        props.fixed_size.then(|| "has-fixed-size"),
    );
    let oninput = props.update.reform(|ev: web_sys::InputEvent| {
        let input: HtmlTextAreaElement = ev.target_dyn_into().expect_throw("event target should be a text area");
        input.value()
    });
    html! {
        <textarea
            name={props.name.clone()}
            value={props.value.clone()}
            {oninput}
            {class}
            rows={props.rows.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
