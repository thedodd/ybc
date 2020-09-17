#![allow(clippy::redundant_closure_call)]

use yew::events::ChangeData;
use yew::prelude::*;
use yewtil::NeqAssign;

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
    pub classes: Option<String>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
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
pub struct Select {
    props: SelectProps,
    link: ComponentLink<Self>,
}

impl Component for Select {
    type Message = String;
    type Properties = SelectProps;

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
        let mut classes = Classes::from("select");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.loading {
            classes.push("is-loading");
        }
        html! {
            <div class=classes>
                <select
                    name=self.props.name.clone()
                    value=self.props.value.clone()
                    onchange=self.link.callback(|change: ChangeData| match change {
                        ChangeData::Select(data) => data.value(),
                        _ => unreachable!("invariant violation: received non-select change event from a select element"),
                    })
                >
                    {self.props.children.clone()}
                </select>
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
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
    pub classes: Option<String>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Size of the list to display.
    #[prop_or_else(|| 4)]
    pub list_size: u32,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
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
pub struct MultiSelect {
    props: MultiSelectProps,
    link: ComponentLink<Self>,
}

impl Component for MultiSelect {
    type Message = Vec<String>;
    type Properties = MultiSelectProps;

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
        let mut classes = Classes::from("select is-multiple");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.loading {
            classes.push("is-loading");
        }
        html! {
            <div class=classes>
                <select
                    multiple=true
                    size=self.props.list_size
                    name=self.props.name.clone()
                    value=self.props.value.join(",")
                    onchange=self.link.callback(|change: ChangeData| match change {
                        ChangeData::Select(data) => {
                            let opts = data.selected_options();
                            (0..opts.length()).into_iter()
                                .filter_map(|idx| opts.item(idx))
                                .filter_map(|elem| elem.get_attribute("value").or_else(|| elem.text_content()))
                                .collect::<Vec<_>>()
                        }
                        _ => unreachable!("invariant violation: received non-select change event from a select element"),
                    })
                >
                    {self.props.children.clone()}
                </select>
            </div>
        }
    }
}
