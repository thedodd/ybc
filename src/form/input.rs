use derive_more::Display;
use yew::prelude::*;
use yew::events::InputData;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| InputType::Text)]
    pub r#type: InputType,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub r#static: bool,
}

pub struct Input {
    props: InputProps,
    link: ComponentLink<Self>,
}

impl Component for Input {
    type Message = String;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{props, link}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.update.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("input");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.rounded {
            classes.push("is-rounded");
        }
        if self.props.loading {
            classes.push("is-loading");
        }
        if self.props.r#static {
            classes.push("is-static");
        }
        html!{
            <input
                name=self.props.name.clone()
                value=self.props.value.clone()
                oninput=self.link.callback(|input: InputData| input.value)
                class=classes
                type=self.props.r#type.to_string()
                placeholder?=self.props.placeholder.clone()
                disabled=self.props.disabled
                readonly=self.props.readonly
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
pub enum InputType {
    #[display(fmt="text")]
    Text,
    #[display(fmt="password")]
    Password,
    #[display(fmt="email")]
    Email,
    #[display(fmt="tel")]
    Tel,
}

/// The three sizes available for an input.
///
/// https://bulma.io/documentation/form/input/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-{}")]
pub enum Size {
    #[display(fmt="small")]
    Small,
    #[display(fmt="medium")]
    Medium,
    #[display(fmt="large")]
    Large,
}
