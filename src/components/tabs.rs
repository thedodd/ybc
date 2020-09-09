use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub boxed: bool,
    #[prop_or_default]
    pub toggle: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub fullwidth: bool,
}

pub struct Tabs {
    props: TabsProps,
}

impl Component for Tabs {
    type Message = ();
    type Properties = TabsProps;

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
        let mut classes = Classes::from("tabs");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if self.props.boxed {
            classes.push("is-boxed");
        }
        if self.props.toggle {
            classes.push("is-toggle");
        }
        if self.props.rounded {
            classes.push("is-rounded");
        }
        if self.props.fullwidth {
            classes.push("is-fullwidth");
        }
        html!{
            <div class=classes>
                <ul>
                    {self.props.children.clone()}
                </ul>
            </div>
        }
    }
}

/// The 2 additional alignments available for tabs.
///
/// https://bulma.io/documentation/components/tabs/#alignment
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-{}")]
pub enum Alignment {
    #[display(fmt="centered")]
    Centered,
    #[display(fmt="right")]
    Right,
}

/// The three sizes available for tabs.
///
/// https://bulma.io/documentation/components/tabs/#sizes
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
