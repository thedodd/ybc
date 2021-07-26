use derive_more::Display;
use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationProps {
    /// The child `li`, `pagination-link` & `pagination-ellipsis` elements for pagination.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// Make the pagination elements rounded.
    #[prop_or_default]
    pub rounded: bool,

    /// The `pagination-previous` element to use.
    pub previous: Html,
    /// The `pagination-next` element to use.
    pub next: Html,
}

/// A responsive, usable, and flexible pagination component.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct Pagination {
    props: PaginationProps,
}

impl Component for Pagination {
    type Message = ();
    type Properties = PaginationProps;

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
        let mut classes = Classes::from("pagination");
        if let Some(extra) = &self.props.classes {
            classes.push(extra);
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
        html! {
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
pub struct PaginationItemProps {
    pub children: Children,
    /// The pagination item type for this component.
    pub item_type: PaginationItemType,
    /// The aria label to use for this element.
    #[prop_or_default]
    pub label: String,
    /// The click handler for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A pagination element representing a link to a page number, the previous page or the next page.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct PaginationItem {
    props: PaginationItemProps,
}

impl Component for PaginationItem {
    type Message = ();
    type Properties = PaginationItemProps;

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
        html! {
            <a class=self.props.item_type.to_string() aria-label=self.props.label.clone() onclick=self.props.onclick.clone()>
                {self.props.children.clone()}
            </a>
        }
    }
}

/// A pagination item type.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "pagination-{}")]
pub enum PaginationItemType {
    /// A pagination link for a specific page number.
    #[display(fmt = "link")]
    Link,
    /// A pagination button for the next page.
    #[display(fmt = "next")]
    Next,
    /// A pagination button for the previous page.
    #[display(fmt = "previous")]
    Previous,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// A horizontal ellipsis for pagination range separators.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct PaginationEllipsis;

impl Component for PaginationEllipsis {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {<span class="pagination-ellipsis">{"&hellip;"}</span>}
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use yew_router::components::RouterAnchor;
    use yew_router::{RouterState, Switch};

    #[derive(Clone, Properties, PartialEq)]
    pub struct RouterProps<SW: Switch + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: SW,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// The pagination item type for this component.
        pub item_type: PaginationItemType,
    }

    /// A Yew Router anchor button for use in a `Pagination` component.
    pub struct PaginationItemRouter<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState = ()> {
        props: RouterProps<SW>,
        marker: std::marker::PhantomData<STATE>,
    }

    impl<SW: Switch + Clone + PartialEq + 'static, STATE: RouterState> Component for PaginationItemRouter<SW, STATE> {
        type Message = ();
        type Properties = RouterProps<SW>;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self { props, marker: std::marker::PhantomData }
        }

        fn update(&mut self, _: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.props.neq_assign(props)
        }

        #[allow(deprecated)]
        fn view(&self) -> Html {
            html! {
                <RouterAnchor<SW, STATE>
                    route=self.props.route.clone()
                    children=self.props.children.clone()
                    classes=self.props.item_type.to_string()
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::PaginationItemRouter;
