use yew::prelude::*;
use yewtil::NeqAssign;

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
    pub classes: Option<String>,
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
pub struct Checkbox {
    props: CheckboxProps,
    link: ComponentLink<Self>,
}

impl Component for Checkbox {
    type Message = bool;
    type Properties = CheckboxProps;

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
        let mut classes = Classes::from("checkbox");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let checked = self.props.checked;
        html! {
            <label class=classes disabled=self.props.disabled>
                <input
                    type="checkbox"
                    checked=self.props.checked
                    name=self.props.name.clone()
                    onclick=self.link.callback(move |_| !checked)
                    disabled=self.props.disabled
                    />
                {self.props.children.clone()}
            </label>
        }
    }
}
