use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub separator: Option<Separator>,
}

pub struct Breadcrumb {
    props: BreadcrumbProps,
}

impl Component for Breadcrumb {
    type Message = ();
    type Properties = BreadcrumbProps;

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
        let mut classes = Classes::from("breadcrumb");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        if let Some(separator) = &self.props.separator {
            classes.push(&separator.to_string());
        }
        html!{
            <nav class=classes aria-label="breadcrumbs">
                <ul>
                    {self.props.children.clone()}
                </ul>
            </nav>
        }
    }
}

/// The three sizes available for a breadcrumb.
///
/// https://bulma.io/documentation/components/breadcrumb/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="are-{}")]
pub enum Size {
    #[display(fmt="small")]
    Small,
    #[display(fmt="medium")]
    Medium,
    #[display(fmt="large")]
    Large,
}

/// The 2 additional alignments available for a breadcrumb.
///
/// https://bulma.io/documentation/components/breadcrumb/#alignment
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-{}")]
pub enum Alignment {
    #[display(fmt="centered")]
    Centered,
    #[display(fmt="right")]
    Right,
}

/// The 4 additional separators for a breadcrump.
///
/// https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="has-{}-separator")]
pub enum Separator {
    #[display(fmt="arrow")]
    Arrow,
    #[display(fmt="bullet")]
    Bullet,
    #[display(fmt="dot")]
    Dot,
    #[display(fmt="succeeds")]
    Succeeds,
}
