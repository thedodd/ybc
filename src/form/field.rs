use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    /// These should all be `Control` components.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub label_classes: Option<String>,
    #[prop_or_default]
    pub help: Option<String>,
    #[prop_or_default]
    pub help_classes: Option<String>,
    #[prop_or_default]
    pub icons_left: bool,
    #[prop_or_default]
    pub icons_right: bool,
    #[prop_or_default]
    pub addons: bool,
    #[prop_or_default]
    pub addons_align: Option<AddonsAlign>,
    #[prop_or_default]
    pub grouped: bool,
    #[prop_or_default]
    pub grouped_align: Option<GroupedAlign>,
    #[prop_or_default]
    pub multiline: bool,
}

pub struct Field {
    props: FieldProps,
}

impl Component for Field {
    type Message = ();
    type Properties = FieldProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("field");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.icons_left {
            classes.push("has-icons-left");
        }
        if self.props.icons_right {
            classes.push("has-icons-right");
        }
        if self.props.addons {
            classes.push("has-addons");
        }
        if let Some(align) = &self.props.addons_align {
            classes.push(&align.to_string());
        }
        if self.props.grouped {
            classes.push("is-grouped");
        }
        if let Some(align) = &self.props.grouped_align {
            classes.push(&align.to_string());
        }
        if self.props.multiline {
            classes.push("is-grouped-multiline");
        }

        // Build the label if label content is provided.
        let label = match &self.props.label {
            Some(label_content) => match &self.props.label_classes {
                Some(label_classes_str) => {
                    let mut label_classes = Classes::from(label_classes_str);
                    label_classes.push("label");
                    html!{<label class=label_classes>{label_content.clone()}</label>}
                }
                None => html!{<label class="label">{label_content.clone()}</label>}
            }
            None => html!{},
        };

        // Build the help label if present.
        let help = match &self.props.help {
            Some(help_content) => match &self.props.help_classes {
                Some(help_classes_str) => {
                    let mut help_classes = Classes::from(help_classes_str);
                    help_classes.push("help");
                    html!{<label class=help_classes>{help_content.clone()}</label>}
                }
                None => html!{<label class="help">{help_content.clone()}</label>}
            }
            None => html!{},
        };

        html!{
            <div class=classes>
                {label}
                {self.props.children.clone()}
                {help}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldHorizontalProps {
    /// The contents of the field body.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub label_size: Option<LabelSize>,
}

/// A field wrapper which causes a field to be horizontal.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
pub struct FieldHorizontal {
    props: FieldHorizontalProps,
}

impl Component for FieldHorizontal {
    type Message = ();
    type Properties = FieldHorizontalProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        // field classes
        let mut classes = Classes::from("field is-horizontal");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }

        // label classes
        let mut labelclasses = Classes::from("field-label");
        if let Some(size) = &self.props.label_size {
            labelclasses.push(&size.to_string());
        }

        html!{
            <div class=classes>
                <div class=labelclasses>
                    <label class="label">{self.props.label.clone()}</label>
                </div>
                <div class="field-body">
                    {self.props.children.clone()}
                </div>
            </div>
        }
    }
}

/// The two alignment options available for field addons.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="has-addons-{}")]
pub enum AddonsAlign {
    #[display(fmt="centered")]
    Centered,
    #[display(fmt="right")]
    Right,
}

/// The two alignment options available for grouped field controls.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-grouped-{}")]
pub enum GroupedAlign {
    #[display(fmt="centered")]
    Centered,
    #[display(fmt="right")]
    Right,
}

/// The three sizes available for horizontal field labels.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-{}")]
pub enum LabelSize {
    #[display(fmt="small")]
    Small,
    #[display(fmt="medium")]
    Medium,
    #[display(fmt="large")]
    Large,
}
