use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CheckboxProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub checked: bool,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The 2-state checkbox in its native format.
///
/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let class = classes!("checkbox", props.classes.clone());
    let checked = props.checked;
    html! {
        <label {class} disabled={props.disabled}>
            <input
                type="checkbox"
                checked={props.checked}
                name={props.name.clone()}
                onclick={props.update.reform(move |_| !checked)}
                disabled={props.disabled}
                />
            {props.children.clone()}
        </label>
    }
}
