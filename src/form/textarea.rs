use derive_more::Display;
use yew::prelude::*;
use yew::events::InputData;
use yewtil::NeqAssign;

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
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub rows: u32,

    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub fixed_size: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub r#static: bool,
}

pub struct TextArea {
    props: TextAreaProps,
    link: ComponentLink<Self>,
}

impl Component for TextArea {
    type Message = String;
    type Properties = TextAreaProps;

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
        html!{
            <textarea
                name=self.props.name.clone()
                value=self.props.value.clone()
                oninput=self.link.callback(|input: InputData| input.value)
                class=classes
                rows=self.props.rows
                placeholder?=self.props.placeholder.clone()
                disabled=self.props.disabled
                readonly=self.props.readonly
                />
        }
    }
}

/// The three sizes available for a textarea.
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
