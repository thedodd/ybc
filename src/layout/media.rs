use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct Media {
    props: MediaProps,
}

impl Component for Media {
    type Message = ();
    type Properties = MediaProps;

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
        let mut classes = Classes::from("media");
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
pub struct MediaLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct MediaLeft {
    props: MediaLeftProps,
}

impl Component for MediaLeft {
    type Message = ();
    type Properties = MediaLeftProps;

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
        let mut classes = Classes::from("media-left");
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
pub struct MediaRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct MediaRight {
    props: MediaRightProps,
}

impl Component for MediaRight {
    type Message = ();
    type Properties = MediaRightProps;

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
        let mut classes = Classes::from("media-right");
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
pub struct MediaContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

pub struct MediaContent {
    props: MediaContentProps,
}

impl Component for MediaContent {
    type Message = ();
    type Properties = MediaContentProps;

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
        let mut classes = Classes::from("media-content");
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
