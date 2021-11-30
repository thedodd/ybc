use derive_more::Display;
use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationProps {
    /// The child `li`, `pagination-link` & `pagination-ellipsis` elements for pagination.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
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
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let class = classes!(
        "pagination",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.alignment.as_ref().map(|alignment| alignment.to_string()),
        props.rounded.then(|| "is-rounded"),
    );
    html! {
        <nav {class} role="navigation" aria-label="pagination">
            {props.previous.clone()}
            {props.next.clone()}
            <ul class="pagination-list">
                {props.children.clone()}
            </ul>
        </nav>
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
#[function_component(PaginationItem)]
pub fn pagination_item(props: &PaginationItemProps) -> Html {
    html! {
        <a class={props.item_type.to_string()} aria-label={props.label.clone()} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </a>
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
#[function_component(PaginationEllipsis)]
pub fn pagination_ellipsis() -> Html {
    html! {<span class="pagination-ellipsis">{"&hellip;"}</span>}
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use serde::Serialize;
    use yew_router::components::Link;
    use yew_router::Routable;

    #[derive(Clone, Properties, PartialEq)]
    pub struct RouterProps<R: Routable + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// The pagination item type for this component.
        pub item_type: PaginationItemType,
    }

    /// A Yew Router anchor button for use in a `Pagination` component.
    pub struct PaginationItemRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for PaginationItemRouter<R, Q> {
        type Message = ();
        type Properties = RouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        #[allow(deprecated)]
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    children={ctx.props().children.clone()}
                    classes={classes!(ctx.props().item_type.to_string())}
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::PaginationItemRouter;
