use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "section".into())]
    pub tag: String,
    /// If you are using a [fixed navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar),
    /// you can use the `fixed_nav=true` modifier on the hero for it to occupy the viewport height minus
    /// the navbar height.
    ///
    /// https://bulma.io/documentation/layout/hero/#fullheight-with-navbar
    #[prop_or_default]
    pub fixed_nav: bool,
}

pub struct Hero {
    props: HeroProps,
}

impl Component for Hero {
    type Message = ();
    type Properties = HeroProps;

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
        let mut classes = Classes::from("hero");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.fixed_nav {
            classes.push("is-fullheight-with-navbar");
        }
        html!{
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroHeadProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct HeroHead {
    props: HeroHeadProps,
}

impl Component for HeroHead {
    type Message = ();
    type Properties = HeroHeadProps;

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
        let mut classes = Classes::from("hero-head");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct HeroBody {
    props: HeroBodyProps,
}

impl Component for HeroBody {
    type Message = ();
    type Properties = HeroBodyProps;

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
        let mut classes = Classes::from("hero-body");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroFootProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct HeroFoot {
    props: HeroFootProps,
}

impl Component for HeroFoot {
    type Message = ();
    type Properties = HeroFootProps;

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
        let mut classes = Classes::from("hero-foot");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}
