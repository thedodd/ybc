use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let class = classes!(
        "select",
        &props.classes,
        props.size.as_ref().map(|size| size.to_string()),
        props.loading.then(|| "is-loading"),
    );
    let onchange = props.update.reform(|ev: web_sys::Event| {
        let select: HtmlSelectElement = ev.target_dyn_into().expect_throw("event target should be a select");
        select.value()
    });
    html! {
        <div {class}>
            <select
                name={props.name.clone()}
                value={props.value.clone()}
                disabled={props.disabled}
                {onchange}
            >
                {props.children.clone()}
            </select>
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: Vec<String>,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<Vec<String>>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Size of the list to display.
    #[prop_or_else(|| 4)]
    pub list_size: u32,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag with the `multiple=true` attribute.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
#[function_component(MultiSelect)]
pub fn multi_select(props: &MultiSelectProps) -> Html {
    let class = classes!(
        "select",
        "is-multiple",
        &props.classes,
        props.size.as_ref().map(|size| size.to_string()),
        props.loading.then(|| "is-loading"),
    );
    let size = props.list_size.to_string();
    let onchange = props.update.reform(|ev: web_sys::Event| {
        let select: HtmlSelectElement = ev.target_dyn_into().expect_throw("event target should be a select");
        let opts = select.selected_options();
        (0..opts.length())
            .into_iter()
            .filter_map(|idx| opts.item(idx))
            .filter_map(|elem| elem.get_attribute("value").or_else(|| elem.text_content()))
            .collect::<Vec<_>>()
    });
    html! {
        <div {class}>
            <select
                multiple=true
                size={size}
                name={props.name.clone()}
                value={props.value.join(",")}
                disabled={props.disabled}
                {onchange}
            >
                {props.children.clone()}
            </select>
        </div>
    }
}
