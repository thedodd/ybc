#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "nav".into())]
    pub tag: String,
}

/// A multi-purpose horizontal level, which can contain almost any other element.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct Level {
    props: LevelProps,
}

impl Component for Level {
    type Message = ();
    type Properties = LevelProps;

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
        let mut classes = Classes::from("level");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html! {
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the left of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelLeft {
    props: LevelLeftProps,
}

impl Component for LevelLeft {
    type Message = ();
    type Properties = LevelLeftProps;

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
        let mut classes = Classes::from("level-left");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html! {
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A container for level elements to be grouped to the right of the container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelRight {
    props: LevelRightProps,
}

impl Component for LevelRight {
    type Message = ();
    type Properties = LevelRightProps;

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
        let mut classes = Classes::from("level-right");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html! {
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// An individual element of a level container.
///
/// [https://bulma.io/documentation/layout/level/](https://bulma.io/documentation/layout/level/)
pub struct LevelItem {
    props: LevelItemProps,
}

impl Component for LevelItem {
    type Message = ();
    type Properties = LevelItemProps;

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
        let mut classes = Classes::from("level-item");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html! {
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}
