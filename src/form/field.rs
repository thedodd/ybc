use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// A text label for the field.
    #[prop_or_default]
    pub label: Option<String>,
    /// Extra classes for the label container.
    #[prop_or_default]
    pub label_classes: Option<Classes>,
    /// A help message for the field.
    #[prop_or_default]
    pub help: Option<String>,
    /// Extra classes for the help message container.
    #[prop_or_default]
    pub help_classes: Option<Classes>,
    /// A convenience bool to add the `is-danger` class to the help classes when `true`.
    #[prop_or_default]
    pub help_has_error: bool,
    /// Has icons on the left of the field's controls.
    #[prop_or_default]
    pub icons_left: bool,
    /// Has icons on the right of the field's controls.
    #[prop_or_default]
    pub icons_right: bool,
    /// Allow addons to the field's controls.
    #[prop_or_default]
    pub addons: bool,
    /// Alignment for the field's addons.
    #[prop_or_default]
    pub addons_align: Option<AddonsAlign>,
    /// All controls in this field should be grouped.
    #[prop_or_default]
    pub grouped: bool,
    /// Alignment for grouped controls.
    #[prop_or_default]
    pub grouped_align: Option<GroupedAlign>,
    /// Allow the grouped controls to span multiple lines.
    #[prop_or_default]
    pub multiline: bool,
    /// Make this a horizontal field.
    #[prop_or_default]
    pub horizontal: bool,
}

/// A container for form controls
#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
    let class = classes!(
        "field",
        props.classes.clone(),
        props.icons_left.then(|| "has-icons-left"),
        props.icons_right.then(|| "has-icons-right"),
        props.addons.then(|| "has-addons"),
        props.grouped.then(|| "is-grouped"),
        props.multiline.then(|| "is-multiline"),
        props.addons_align.as_ref().map(|align| align.to_string()),
        props.grouped_align.as_ref().map(|align| align.to_string()),
    );

    // Build the label if label content is provided.
    let label = match &props.label {
        Some(label_content) => match &props.label_classes {
            Some(label_classes_str) => {
                let mut label_classes = label_classes_str.clone();
                if props.horizontal {
                    label_classes.push("field-label");
                    html! {
                        <div class={label_classes}>
                            <label class="label">{label_content.clone()}</label>
                        </div>
                    }
                } else {
                    label_classes.push("label");
                    html! {<label class={label_classes}>{label_content.clone()}</label>}
                }
            }
            None => {
                if props.horizontal {
                    html! {<div class="field-label"><label class="label">{label_content.clone()}</label></div>}
                } else {
                    html! {<label class="label">{label_content.clone()}</label>}
                }
            }
        },
        None => html! {},
    };

    // Build the help label if present.
    let help = match &props.help {
        Some(help_content) => match &props.help_classes {
            Some(help_classes_str) => {
                let class = classes!("help", help_classes_str.clone(), props.help_has_error.then(|| "is-danger"));
                html! {<label {class}>{help_content.clone()}</label>}
            }
            None => {
                let class = classes!("help", props.help_has_error.then(|| "is-danger"));
                html! {<label {class}>{help_content.clone()}</label>}
            }
        },
        None => html! {},
    };

    // Build the body section.
    let body = if props.horizontal {
        html! {<div class="field-body">{props.children.clone()}</div>}
    } else {
        html! {<>{props.children.clone()}</>}
    };

    html! {
        <div {class}>
            {label}
            {body}
            {help}
        </div>
    }
}

/// The two alignment options available for field addons.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "has-addons-{}")]
pub enum AddonsAlign {
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// The two alignment options available for grouped field controls.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-grouped-{}")]
pub enum GroupedAlign {
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// The three sizes available for horizontal field labels.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum LabelSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}
