use yew::events::InputData;
use yew::prelude::*;
use yewtil::NeqAssign;

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
    pub classes: Option<String>,
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
pub struct TextArea {
    props: TextAreaProps,
    link: ComponentLink<Self>,
}

impl Component for TextArea {
    type Message = String;
    type Properties = TextAreaProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.update.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("textarea");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.loading {
            classes.push("is-loading");
        }
        if self.props.r#static {
            classes.push("is-static");
        }
        if self.props.fixed_size {
            classes.push("has-fixed-size");
        }
        html! {
            <textarea
                name=self.props.name.clone()
                value=self.props.value.clone()
                oninput=self.link.callback(|input: InputData| input.value)
                class=classes
                rows=self.props.rows
                placeholder=self.props.placeholder.clone()
                disabled=self.props.disabled
                readonly=self.props.readonly
                />
        }
    }
}
