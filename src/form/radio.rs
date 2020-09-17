use yew::events::InputData;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RadioProps {
    /// The `name` attribute for this form element.
    ///
    /// All members of the same radio group must have the same value for their `name` attribute.
    pub name: String,
    /// The `value` attribute for this form element.
    ///
    /// This is different from other form elements, as this value does not change. It represents
    /// the value to be used for the radio group overall when this element is selected.
    pub value: String,
    /// The value of the currently selected radio of this radio group.
    pub checked_value: Option<String>,
    /// The callback to be used for propagating changes to the selected radio of the radio group.
    pub update: Callback<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The mutually exclusive radio buttons in their native format.
///
/// [https://bulma.io/documentation/form/radio/](https://bulma.io/documentation/form/radio/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Radio {
    props: RadioProps,
    link: ComponentLink<Self>,
}

impl Component for Radio {
    type Message = String;
    type Properties = RadioProps;

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
        let mut classes = Classes::from("radio");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html! {
            <label class=classes disabled=self.props.disabled>
                <input
                    type="radio"
                    name=self.props.name.clone()
                    value=self.props.value.clone()
                    checked=self.props.checked_value.as_ref().map(|val| val == &self.props.value).unwrap_or(false)
                    oninput=self.link.callback(|data: InputData| data.value)
                    disabled=self.props.disabled
                    />
                {self.props.children.clone()}
            </label>
        }
    }
}
