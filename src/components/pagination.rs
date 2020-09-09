use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationProps {
    /// The child `li`, `pagination-link` & `pagination-ellipsis` elements for pagination.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    #[prop_or_default]
    pub rounded: bool,

    /// The `pagination-previous` element to use.
    pub previous: Html,
    /// The `pagination-next` element to use.
    pub next: Html,
}

/// A responsive, usable, and flexible pagination component.
///
/// https://bulma.io/documentation/components/pagination/
pub struct Pagination {
    props: PaginationProps,
}

impl Component for Pagination {
    type Message = ();
    type Properties = PaginationProps;

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
        let mut classes = Classes::from("pagination");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &self.props.alignment {
            classes.push(&alignment.to_string());
        }
        if self.props.rounded {
            classes.push("is-rounded");
        }
        html!{
            <nav class=classes role="navigation" aria-label="pagination">
                {self.props.previous.clone()}
                {self.props.next.clone()}
                <ul class="pagination-list">
                    {self.props.children.clone()}
                </ul>
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationLinkProps {
    pub children: Children,
    /// The aria label to use for this element.
    #[prop_or_default]
    pub label: String,
}

pub struct PaginationLink {
    props: PaginationLinkProps,
}

impl Component for PaginationLink {
    type Message = ();
    type Properties = PaginationLinkProps;

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
        html!{
            <a class="pagination-link" aria-label=self.props.label.clone()>
                {self.props.children.clone()}
            </a>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

pub struct PaginationEllipsis;

impl Component for PaginationEllipsis {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{<span class="pagination-ellipsis">{"&hellip;"}</span>}
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// The three sizes available for a pagination component.
///
/// https://bulma.io/documentation/components/pagination/#sizes
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

/// The different alignments available for pagination components.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt="is-{}")]
pub enum Alignment {
    #[display(fmt="centered")]
    Centered,
    #[display(fmt="right")]
    Right,
}
