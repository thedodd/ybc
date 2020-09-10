use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub classes: Option<String>,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_default]
    pub max: Option<f32>,
    /// The amount of progress which has been made.
    #[prop_or_default]
    pub value: Option<f32>,
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
        Self{props}
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
        let max = self.props.max.as_ref().map(|val| val.to_string());
        let value = self.props.value.as_ref().map(|val| val.to_string());
        let value_txt = value.as_ref()
            .map(|val| html!{{format!("{}%", val)}})
            .unwrap_or_else(|| html!{});
        html!{
            <progress class=classes max?=max value?=value>
                {value_txt}
            </progress>
        }
    }
}
