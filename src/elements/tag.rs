use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "span".into())]
    pub tag: String,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub is_delete: bool,
}

pub struct Tag {
    props: TagProps,
}

impl Component for Tag {
    type Message = ();
    type Properties = TagProps;

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
        let mut classes = Classes::from("tag");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.is_delete {
            classes.push("is-delete");
        }
        let tag = self.props.tag.clone();
        html!{
            <@{tag} class=classes onclick?=self.props.onclick.clone()>
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
    #[prop_or_default]
    pub has_addons: bool,
}

pub struct Tags {
    props: TagsProps,
}

impl Component for Tags {
    type Message = ();
    type Properties = TagsProps;

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
        let mut classes = Classes::from("tags");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.has_addons {
            classes.push("has-addons");
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}
