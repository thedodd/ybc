use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// A size modifier to control spacing.
    #[prop_or_default]
    pub size: Option<SectionSize>,
}

/// A simple container to divide your page into sections.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
pub struct Section {
    props: SectionProps,
}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

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
        let mut classes = Classes::from("section");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        html! {
            <section class=classes>
                {self.props.children.clone()}
            </section>
        }
    }
}

/// The 2 sizes available for sections, which controls spacing.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum SectionSize {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}
