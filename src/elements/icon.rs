use derive_more::Display;
use yew::prelude::*;
use yew::events::MouseEvent;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

pub struct Icon {
    props: IconProps,
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

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
        let mut classes = Classes::from("icon");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        html!{
            <span class=classes onclick?=self.props.onclick.clone()>
                {self.props.children.clone()}
            </span>
        }
    }
}

/// Available sizes for the icon container, to help prevent page "jumps" during load.
///
/// The set a "normal" size, simply omit the size parameter altogether, as "normal" is the default.
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

/// Possible icon alignments, often used within form controls.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-{}")]
pub enum Alignment {
    #[display(fmt="left")]
    Left,
    #[display(fmt="right")]
    Right,
}
