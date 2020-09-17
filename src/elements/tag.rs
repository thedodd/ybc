#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yewtil::NeqAssign;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "span".into())]
    pub tag: String,
    /// The click handler for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Make this tag rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Turn this tag into a delete button.
    #[prop_or_default]
    pub delete: bool,
    /// The size for this component.
    #[prop_or_default]
    pub size: Option<Size>,
}

/// A small tag label to insert anywhere.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
pub struct Tag {
    props: TagProps,
}

impl Component for Tag {
    type Message = ();
    type Properties = TagProps;

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
        let mut classes = Classes::from("tag");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.rounded {
            classes.push("is-rounded");
        }
        if self.props.delete {
            classes.push("is-delete");
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        let tag = self.props.tag.clone();
        html! {
            <@{tag} class=classes onclick=self.props.onclick.clone()>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// Attach two tags together; this requires that this component wraps two `Tag` components.
    #[prop_or_default]
    pub has_addons: bool,
}

/// A container for a list of tags.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
pub struct Tags {
    props: TagsProps,
}

impl Component for Tags {
    type Message = ();
    type Properties = TagsProps;

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
        let mut classes = Classes::from("tags");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.has_addons {
            classes.push("has-addons");
        }
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}
