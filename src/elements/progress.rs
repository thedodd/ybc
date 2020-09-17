#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub classes: Option<String>,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made.
    #[prop_or_else(|| 0.0)]
    pub value: f32,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
pub struct Progress {
    props: ProgressProps,
}

impl Component for Progress {
    type Message = ();
    type Properties = ProgressProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("progress");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let max = self.props.max.to_string();
        let value = self.props.value.to_string();
        let value_txt = html! {{format!("{}%", value)}};
        html! {
            <progress class=classes max=max value=value>
                {value_txt}
            </progress>
        }
    }
}
